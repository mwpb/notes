defmodule BasicBlocks do
  def basic_blocks_from_programme(programme) do
    programme["functions"] |> Enum.flat_map(&basic_blocks_from_function/1)
  end

  def basic_blocks_from_function(function) do
    function["instrs"] |> basic_blocks_from_instrs([], [])
  end

  def basic_blocks_from_instrs([] = _instrs, [] = _current_block, basic_blocks) do
    Enum.reverse(basic_blocks)
  end

  def basic_blocks_from_instrs([] = _instrs, current_block, basic_blocks) do
    basic_blocks_from_instrs([], [], [Enum.reverse(current_block) | basic_blocks])
  end

  def basic_blocks_from_instrs([h | t] = _instrs, current_block, basic_blocks) do
    cond do
      ControlUtils.is_terminator(h["op"]) ->
        basic_blocks_from_instrs(t, [], [Enum.reverse([h | current_block]) | basic_blocks])

      true ->
        basic_blocks_from_instrs(t, [h | current_block], basic_blocks)
    end
  end
end
