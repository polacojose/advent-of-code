defmodule Instruction do
  defstruct name: "", op1: nil, op2: nil

  def parse_instructions(file) do
    File.stream!(file) |> Enum.map(&parse_instruction/1)
  end

  def parse_instruction(inst_line) do
    result =
      Regex.run(~r/(\w+)\s+([\w-+\d]+)(, (\S+))?/, String.trim(inst_line),
        capture: :all_but_first
      )

    case result do
      ["hlf", op1] -> %Instruction{name: "hlf", op1: op1}
      ["tpl", op1] -> %Instruction{name: "tpl", op1: op1}
      ["inc", op1] -> %Instruction{name: "inc", op1: op1}
      ["jmp", op1] -> %Instruction{name: "jmp", op1: op1}
      ["jie", op1, _, op2] -> %Instruction{name: "jie", op1: op1, op2: op2}
      ["jio", op1, _, op2] -> %Instruction{name: "jio", op1: op1, op2: op2}
    end
  end
end
