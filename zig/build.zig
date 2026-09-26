const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{
        .default_target = .{
            .os_tag = .macos,
            .os_version_min = .{ .semver = .{ .major = 15, .minor = 4, .patch = 0 } },
        },
    });

    const exe = b.addExecutable(.{
        .name = "codeabby",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = b.standardOptimizeOption(.{}),
    });
    b.installArtifact(exe);

    const run = b.addRunArtifact(exe);
    if (b.args) |args| run.addArgs(args);

    const run_step = b.step("run", "Run the current problem");
    run_step.dependOn(&run.step);
}
