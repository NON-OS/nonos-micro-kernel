#!/usr/bin/env ruby
#
# Benchmark Jsonxf against other JSON processors at the command line.
# Mostly for internal use.

PP_INPUT = ENV["PP_INPUT"]
raise "PP_INPUT must be provided" unless PP_INPUT

MIN_INPUT = ENV["MIN_INPUT"]
raise "MIN_INPUT must be provided" unless MIN_INPUT

OUTPUT = ENV["OUTPUT"] || "/tmp/out.json"

SKIP_JQ = !!ENV["SKIP_JQ"]

TIME_REGEX = %r/
  real \s+ (?<min>\d+) m (?<sec>\d+) \. (?<msec>\d+) s
/x;

def time_command_once(cmd)
  if m = TIME_REGEX.match(`time (#{cmd}) 2>&1`)
    m[:min].to_i*60 + m[:sec].to_i + m[:msec].to_f/1000.0
  else
    nil
  end
end

def time_command(cmd)
  times = [
    time_command_once(cmd),
    time_command_once(cmd),
    time_command_once(cmd),
    time_command_once(cmd),
    time_command_once(cmd),
  ]
  times.sort!
  (times[1] + times[2] + times[3]) / 3
end

def time_str(cmd_part)
  cmd = "#{cmd_part} <'#{PP_INPUT}' >'#{OUTPUT}'"
  time = time_command(cmd)
  "| `#{cmd_part}` | #{sprintf("%.2f", time)} |"
end


puts "Pretty-print test:"
puts time_str("cat")
puts time_str("../target/release/jsonxf")
puts time_str("./serdexf/target/release/serdexf")
puts time_str("jsonpp")
puts time_str("jq -M .") unless SKIP_JQ

puts ""

puts "Minimize test:"
puts time_str("cat")
puts time_str("../target/release/jsonxf -m")
puts time_str("./serdexf/target/release/serdexf -m")
puts time_str("jq -cM .") unless SKIP_JQ

