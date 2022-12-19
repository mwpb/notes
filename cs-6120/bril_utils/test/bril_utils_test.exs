defmodule BrilUtilsTest do
  use ExUnit.Case
  doctest BrilUtils

  test "greets the world" do
    assert BrilUtils.hello() == :world
  end
end
