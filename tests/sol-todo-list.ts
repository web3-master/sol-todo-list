import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolTodoList } from "../target/types/sol_todo_list";

describe("sol-todo-list", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolTodoList as Program<SolTodoList>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
