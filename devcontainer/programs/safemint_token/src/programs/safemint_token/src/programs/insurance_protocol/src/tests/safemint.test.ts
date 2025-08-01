import * as anchor from "@coral-xyz/anchor";
import { assert } from "chai";

describe("safemint", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  it("boots", async () => {
    assert.ok(true, "Workspace initialized");
  });
});
