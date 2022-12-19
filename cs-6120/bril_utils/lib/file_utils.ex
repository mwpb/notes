defmodule FileUtils do
  @spec load(String.t()) :: term
  def load(filepath) do
    file = File.read!(filepath)
    Jason.decode!(file)
  end
end
