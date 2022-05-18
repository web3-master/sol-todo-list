import * as anchor from "@project-serum/anchor";
import { AnchorError, Program } from "@project-serum/anchor";
import { assert, expect } from "chai";
import { SolTodoList } from "../target/types/sol_todo_list";
import BN from "bn.js";
const { SystemProgram, LAMPORTS_PER_SOL } = anchor.web3;

const provider = anchor.AnchorProvider.env();
anchor.setProvider(anchor.AnchorProvider.env());
const mainProgram = anchor.workspace.SolTodoList as Program<SolTodoList>;

async function getAccountBalance(pubKey) {
  let account = await provider.connection.getAccountInfo(pubKey);
  return account?.lamports ?? 0;
}

async function programForUser(user) {
  return new anchor.Program(
    mainProgram.idl,
    mainProgram.programId,
    user.provider
  );
}

async function createUser(airdropBalance?: number) {
  airdropBalance = airdropBalance ?? 10 * LAMPORTS_PER_SOL;
  let user = anchor.web3.Keypair.generate();
  const sig = await provider.connection.requestAirdrop(
    user.publicKey,
    airdropBalance
  );
  await provider.connection.confirmTransaction(sig);

  let wallet = new anchor.Wallet(user);
  let userProvider = new anchor.AnchorProvider(
    provider.connection,
    wallet,
    provider.opts
  );

  return {
    key: user,
    wallet,
    provider: userProvider,
  };
}

async function createUsers(numUsers) {
  let promises = [];
  for (let i = 0; i < numUsers; i++) {
    promises.push(createUser());
  }
  return Promise.all(promises);
}

async function createList(owner: any, name: string, capacity = 16) {
  const [listAccount, bump] = await anchor.web3.PublicKey.findProgramAddress(
    ["todolist", owner.key.publicKey.toBytes(), name.slice(0, 32)],
    mainProgram.programId
  );

  let program = await programForUser(owner);
  await program.methods
    .createList(name, capacity, bump)
    .accounts({
      list: listAccount,
      user: owner.key.publicKey,
    })
    .rpc();

  let list = await program.account.todoList.fetch(listAccount);
  return {
    publicKey: listAccount,
    data: list,
  };
}

async function addItem(list, user, name, bounty) {
  const itemAccount = anchor.web3.Keypair.generate();
  let program = await programForUser(user);
  await program.methods
    .add(list.data.name, name, new BN(bounty))
    .accounts({
      list: list.publicKey,
      listOwner: list.data.listOwner,
      item: itemAccount.publicKey,
      user: user.key.publicKey,
    })
    .signers([itemAccount])
    .rpc();

  let listData = await program.account.todoList.fetch(list.publicKey);
  let itemData = await program.account.listItem.fetch(itemAccount.publicKey);

  return {
    list: {
      publicKey: list.publicKey,
      data: listData,
    },
    item: {
      publicKey: itemAccount.publicKey,
      data: itemData,
      itemAccount: itemAccount,
    },
  };
}

async function cancelItem(list, item, itemCreator, user) {
  let program = await programForUser(user);
  await program.methods
    .cancel(list.data.name)
    .accounts({
      list: list.publicKey,
      listOwner: list.data.listOwner,
      item: item.publicKey,
      itemCreator: itemCreator.key.publicKey,
      user: user.key.publicKey,
    })
    .signers([item.itemAccount])
    .rpc();

  let listData = await program.account.todoList.fetch(list.publicKey);

  return {
    publicKey: list.publicKey,
    data: listData,
  };
}

describe("create list", () => {
  it("create a list", async () => {
    const owner = await createUser();
    const list = await createList(owner, "A list");

    expect(list.data.listOwner.toString()).equals(
      owner.key.publicKey.toString()
    );
    expect(list.data.name).equals("A list");
    expect(list.data.lines.length).equals(0);
  });
});

describe("add item", () => {
  it("add items", async () => {
    const [owner, adder] = await createUsers(2);

    const list = await createList(owner, "list");
    const result = await addItem(
      list,
      adder,
      "Do something",
      1 * LAMPORTS_PER_SOL
    );

    expect(result.list.data.lines).eql([result.item.publicKey]);
    expect(result.item.data.creator.toString()).equals(
      adder.key.publicKey.toString()
    );
    expect(result.item.data.creatorFinished).equals(false);
    expect(result.item.data.listOwnerFinished).equals(false);
    expect(result.item.data.name).equals("Do something");
    expect(await getAccountBalance(result.item.publicKey)).equals(
      1 * LAMPORTS_PER_SOL
    );

    // Test that another add works.
    const again = await addItem(
      list,
      adder,
      "Another item",
      1 * LAMPORTS_PER_SOL
    );
    expect(again.list.data.lines).eql([
      result.item.publicKey,
      again.item.publicKey,
    ]);
  });

  it("fails if the bounty is too small", async () => {
    const [owner, adder] = await createUsers(2);

    const list = await createList(owner, "list", 2);

    try {
      await addItem(list, adder, "Third item", 10);
      expect.fail("Item add should be failed because bounty was too small!");
    } catch (err) {
      expect(err).to.be.instanceOf(AnchorError);
      const anchorError = err as AnchorError;
      expect(anchorError.error.errorCode.number).equals(6002);
      expect(anchorError.error.errorCode.code).equals("BountyTooSmall");
    }
  });

  it("fails if the list is full", async () => {
    const [owner, adder] = await createUsers(2);

    const list = await createList(owner, "list", 2);
    const result = await addItem(
      list,
      adder,
      "First item",
      1 * LAMPORTS_PER_SOL
    );

    expect(result.list.data.lines).eql([result.item.publicKey]);
    expect(result.item.data.creator.toString()).equals(
      adder.key.publicKey.toString()
    );
    expect(result.item.data.creatorFinished).equals(false);
    expect(result.item.data.listOwnerFinished).equals(false);
    expect(result.item.data.name).equals("First item");
    expect(await getAccountBalance(result.item.publicKey)).equals(
      1 * LAMPORTS_PER_SOL
    );

    const second = await addItem(
      list,
      adder,
      "Second item",
      1 * LAMPORTS_PER_SOL
    );
    expect(second.list.data.lines).eql([
      result.item.publicKey,
      second.item.publicKey,
    ]);

    try {
      const third = await addItem(
        list,
        adder,
        "Third item",
        1 * LAMPORTS_PER_SOL
      );
      expect(third.list.data.lines).eql([
        result.item.publicKey,
        second.item.publicKey,
        third.item.publicKey,
      ]);
      assert(false, "Third item add should be failed!");
    } catch (err) {
      expect(err).to.be.instanceOf(AnchorError);
      const anchorError = err as AnchorError;
      expect(anchorError.error.errorCode.number).equals(6000);
      expect(anchorError.error.errorCode.code).equals("ListFull");
    }
  });
});

describe("cancel item", () => {
  it("add an item and cancel it", async () => {
    const [owner, adder] = await createUsers(2);

    const list = await createList(owner, "list");

    const adderStartingBalance = await getAccountBalance(adder.key.publicKey);
    console.log("adderStartingBalance", adderStartingBalance);

    const result = await addItem(
      list,
      adder,
      "Do something",
      1 * LAMPORTS_PER_SOL
    );

    const adderBalanceAfterAdd = await getAccountBalance(adder.key.publicKey);
    console.log("adderBalanceAfterAdd", adderBalanceAfterAdd);

    expect(result.list.data.lines).eql([result.item.publicKey]);

    // Cancel this item.
    const resultAfterCancel = await cancelItem(list, result.item, adder, adder);
    expect(resultAfterCancel.data.lines).eql([]);

    const adderBalanceAfterCancel = await getAccountBalance(
      adder.key.publicKey
    );
    console.log("adderBalanceAfterCancel", adderBalanceAfterCancel);

    expect(adderBalanceAfterCancel).equals(adderStartingBalance);
  });
});
