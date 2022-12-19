defmodule GenerateCfgTest do
  use ExUnit.Case

  test "test_test" do
    add_prog = FileUtils.load("../../../bril/test/parse/add.json")
    IO.inspect(add_prog)
    assert true
  end
end
