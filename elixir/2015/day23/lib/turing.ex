defmodule Turing do
  require Integer
  use Agent
  @type registers :: %{}
  @valid_registers ["a", "b", "ip"]

  @spec set_register(registers(), String.t(), non_neg_integer()) :: {:ok, registers()} | :error
  defp set_register(_, reg, _) when reg not in @valid_registers, do: :error
  defp set_register(_, _, val) when not is_integer(val) or val < 0, do: :error

  defp set_register(registers, reg, val) do
    {:ok, Map.put(registers, reg, val)}
  end

  @spec get_register(registers(), String.t()) :: non_neg_integer()
  defp get_register(registers, reg) do
    Map.get(registers, reg, 0)
  end

  @spec get_val(registers(), String.t() | integer()) :: non_neg_integer()
  defp get_val(registers, val) do
    result = Integer.parse(val)

    case result do
      {val, _} -> val
      _ -> get_register(registers, val)
    end
  end

  @spec new() :: {:ok, pid()}
  def new() do
    Agent.start_link(fn -> %{} end)
  end

  def registers(pid) do
    Agent.get(pid, &Function.identity/1)
  end

  @spec execute_program(pid(), [%Instruction{}]) :: :ok
  def execute_program(pid, instructions) do
    registers = Agent.get(pid, &Function.identity/1)
    ip = get_register(registers, "ip")
    instruction = Enum.at(instructions, ip)

    case instruction do
      nil ->
        :ok

      _ ->
        :ok = execute_instruction(pid, instruction)
        execute_program(pid, instructions)
    end
  end

  @spec execute_instruction(pid(), %Instruction{}) :: :ok
  defp execute_instruction(pid, instruction) do
    registers = Agent.get(pid, &Function.identity/1)

    {:ok, registers} =
      case instruction do
        %Instruction{name: "hlf", op1: op1} ->
          val = get_register(registers, op1)
          {:ok, registers} = set_register(registers, op1, div(val, 2))

          ip = get_register(registers, "ip")
          set_register(registers, "ip", ip + 1)

        %Instruction{name: "tpl", op1: op1} ->
          val = get_register(registers, op1)
          {:ok, registers} = set_register(registers, op1, val * 3)

          ip = get_register(registers, "ip")
          set_register(registers, "ip", ip + 1)

        %Instruction{name: "inc", op1: op1} ->
          val = get_val(registers, op1)
          {:ok, registers} = set_register(registers, op1, val + 1)

          ip = get_register(registers, "ip")
          set_register(registers, "ip", ip + 1)

        %Instruction{name: "jmp", op1: op1} ->
          op1 = get_val(registers, op1)

          ip = get_register(registers, "ip")
          set_register(registers, "ip", ip + op1)

        %Instruction{name: "jie", op1: op1, op2: op2} ->
          op1 = get_val(registers, op1)
          op2 = get_val(registers, op2)

          ip = get_register(registers, "ip")

          if Integer.is_even(op1) do
            set_register(registers, "ip", ip + op2)
          else
            set_register(registers, "ip", ip + 1)
          end

        %Instruction{name: "jio", op1: op1, op2: op2} ->
          op1 = get_val(registers, op1)
          op2 = get_val(registers, op2)

          ip = get_register(registers, "ip")

          if op1 == 1 do
            set_register(registers, "ip", ip + op2)
          else
            set_register(registers, "ip", ip + 1)
          end
      end

    Agent.update(pid, fn _ -> registers end)
    :ok
  end
end
