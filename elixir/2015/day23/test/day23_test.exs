defmodule Day23Test do
  use ExUnit.Case
  doctest Day23

  alias Instruction, as: I

  test "Instruction parsing" do
    assert I.parse_instruction("hlf a") == %I{name: "hlf", op1: "a"}
    assert I.parse_instruction("tpl a") == %I{name: "tpl", op1: "a"}
    assert I.parse_instruction("inc a") == %I{name: "inc", op1: "a"}
    assert I.parse_instruction("jmp +1") == %I{name: "jmp", op1: "+1"}
    assert I.parse_instruction("jie a, +1") == %I{name: "jie", op1: "a", op2: "+1"}
    assert I.parse_instruction("jio a, +1") == %I{name: "jio", op1: "a", op2: "+1"}
  end

  test "Turing inc" do
    {:ok, t} = Turing.new()
    i = [I.parse_instruction("inc a")]
    Turing.execute_program(t, i)
    registers = Turing.registers(t)
    a = Map.get(registers, "a")
    ip = Map.get(registers, "ip")
    assert(a == 1)
    assert(ip == 1)

    i = [I.parse_instruction("inc a"), I.parse_instruction("inc a")]
    Turing.execute_program(t, i)
    registers = Turing.registers(t)
    a = Map.get(registers, "a")
    ip = Map.get(registers, "ip")
    assert(a == 2)
    assert(ip == 2)
  end
end
