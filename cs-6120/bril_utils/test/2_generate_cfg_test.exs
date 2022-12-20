defmodule GenerateCfgTest do
  use ExUnit.Case

  test "test_test" do
    add_prog = FileUtils.load("./test/json/jmp.json")
    basic_blocks = BasicBlocks.basic_blocks_from_programme(add_prog)
    IO.inspect(basic_blocks)
    assert true
  end
end
