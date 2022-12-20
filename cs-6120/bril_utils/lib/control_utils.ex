defmodule ControlUtils do
  @spec is_terminator(String.t()) :: boolean
  def is_terminator(op) do
    terminators = MapSet.new(["jmp", "br", "ret"])
    MapSet.member?(terminators, op)
  end

  @spec is_control(String.t()) :: boolean
  def is_control(op) do
    non_terminators = MapSet.new(["call"])
    is_terminator(op) || MapSet.member?(non_terminators, op)
  end
end
