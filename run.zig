const std = @import("std");

pub fn main() anyerror!void {
    const exe_path = FilePath{"path/to/program.exe"};
    const args = [_][]const u8{"arg1", "arg2", "arg3"};

    const process = try std.proc.spawnProcess(exe_path, args, .{});
    const status_code = try process.wait();

    std.debug.print("Program exited with status code {}\n", status_code);

    return;
}
