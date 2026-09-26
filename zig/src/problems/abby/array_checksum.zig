const std = @import("std");

pub fn checksum(values: anytype) u64 {
    var result: u64 = 0;
    for (values) |value| {
        result = ((result + @as(u64, @intCast(value))) * 113) % 10_000_007;
    }
    return result;
}

fn readUnsigned(reader: anytype, comptime T: type) !T {
    var value: T = 0;
    var started = false;

    while (true) {
        const byte = reader.readByte() catch |err| switch (err) {
            error.EndOfStream => return if (started) value else err,
            else => return err,
        };
        if (std.ascii.isWhitespace(byte)) {
            if (started) return value;
        } else if (std.ascii.isDigit(byte)) {
            value = value * 10 + @as(T, byte - '0');
            started = true;
        } else {
            return error.InvalidCharacter;
        }
    }
}

pub fn main() !void {
    var buffered = std.io.bufferedReader(std.io.getStdIn().reader());
    const reader = buffered.reader();
    const allocator = std.heap.page_allocator;

    const n = try readUnsigned(reader, u32);

    const values = try allocator.alloc(u64, n);
    defer allocator.free(values);

    for (values) |*value| {
        value.* = try readUnsigned(reader, u64);
    }

    const result = checksum(values);
    std.debug.print("{}\n", .{result});
}
