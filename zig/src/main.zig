const std = @import("std");
const problems = @import("root.zig");

const Problem = struct {
    number: u32,
    name: []const u8,
    run: *const fn () anyerror!void,
};

// To add a problem, export its module from the matching problems/<author>/root.zig,
// then add one entry here.
const problem_registry = [_]Problem{
    .{
        .number = 17,
        .name = "Array Checksum ✰",
        .run = problems.abby.array_checksum.main,
    },
    .{
        .number = 23,
        .name = "Bubble in Array ✰",
        .run = problems.abby.bubble_in_array.main,
    },
};

pub fn main() !void {
    std.debug.print("Select a problem:\n\n", .{});
    for (problem_registry) |problem| {
        std.debug.print("  [{d}] {s}\n", .{ problem.number, problem.name });
    }
    std.debug.print("\nProblem number: ", .{});

    const stdin = std.io.getStdIn().reader();
    const allocator = std.heap.page_allocator;
    const line = try stdin.readUntilDelimiterAlloc(allocator, '\n', 32);
    defer allocator.free(line);
    const number = try std.fmt.parseInt(u32, std.mem.trim(u8, line, " \t\r"), 10);

    for (problem_registry) |problem| {
        if (problem.number == number) return problem.run();
    }

    return error.UnknownProblem;
}
