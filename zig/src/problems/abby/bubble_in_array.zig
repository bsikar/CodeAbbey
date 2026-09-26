const std = @import("std");
const array_checksum = @import("array_checksum.zig");

fn readU32(reader: anytype) !?u32 {
    var byte = reader.readByte() catch |err| switch (err) {
        error.EndOfStream => return null,
        else => return err,
    };

    while (std.ascii.isWhitespace(byte)) {
        byte = reader.readByte() catch |err| switch (err) {
            error.EndOfStream => return null,
            else => return err,
        };
    }

    if (!std.ascii.isDigit(byte)) return null;

    var value: u32 = 0;
    while (std.ascii.isDigit(byte)) {
        value = value * 10 + @as(u32, byte - '0');
        byte = reader.readByte() catch |err| switch (err) {
            error.EndOfStream => return value,
            else => return err,
        };
    }

    return value;
}

fn groupBy2(comptime T: type, items: []T) struct { left: []T, right: []T } {
    if (items.len < 2) {
        return .{ .left = items[0..0], .right = items[0..0] };
    }

    return .{
        .left = items[0 .. items.len - 1],
        .right = items[1..],
    };
}

pub fn bubbleInArray(comptime T: type, items: []T) struct { u32, u64 } {
    var numSwapped: u32 = 0;
    const pairs = groupBy2(T, items);

    for (pairs.left, pairs.right) |*left, *right| {
        if (left.* > right.*) {
            numSwapped += 1;
            const temp = left.*;
            left.* = right.*;
            right.* = temp;
        }
    }

    return .{ numSwapped, array_checksum.checksum(items) };
}

pub fn main() !void {
    var buffered = std.io.bufferedReader(std.io.getStdIn().reader());
    const reader = buffered.reader();
    const allocator = std.heap.page_allocator;

    var values = std.ArrayList(u32).init(allocator);
    defer values.deinit();

    while (try readU32(reader)) |value| {
        try values.append(value);
    }

    const result = bubbleInArray(u32, values.items);
    std.debug.print("{} {}\n", .{ result[0], result[1] });
}
