import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaBlog } from "../target/types/solana_blog";

import { assert } from "chai";
import * as web3 from "@solana/web3.js";
import { ASSOCIATED_PROGRAM_ID } from "@project-serum/anchor/dist/cjs/utils/token";

interface User {
  authority: web3.Keypair,
  userProfile: web3.PublicKey,
}

interface Blog {
  authority: web3.Keypair,
  userProfile: web3.PublicKey,
  blogAccount: web3.PublicKey,
}

describe("solana_blog", () => {
 
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  let connection = provider.connection;
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaBlog as Program<SolanaBlog>;

  const name = "Satyam";
  const profile_image = "some profile image url"

  it("initialize user", async () => {
    // address of authority
    let authority = provider.wallet.publicKey;
    // address of userProfile
    let [userProfile, userProfile_b] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("USER_STATE"), authority.toBuffer()], 
      program.programId,
    );

    // call the initialize user function to test
    const tx = await program.rpc.initializeUser(
      name,
      profile_image,
      {
        accounts: {
          authority: authority,
          userProfile: userProfile,
          systemProgram: web3.SystemProgram.programId
        }
      }
    );

    // print the signature of transaction
    console.log("Your transaction signature", tx);
  });

  it("add a new blog", async () => {
    const title = "some title";
    const content = "some content";
    const blog_image = "some image url";

    // address of authority
    let authority = provider.wallet.publicKey;

    // address of userProfile
    let [userProfile, userProfile_b] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("USER_STATE"), authority.toBuffer()], 
      program.programId,
    );

    // address of userProfile
    let [blogAccount, blogAccount_b] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("BLOG_STATE"), authority.toBuffer(), new anchor.BN(0).toBuffer()], 
      program.programId,
    );

    // call the function to add blog
    const tx = await program.rpc.addBlog(
      title,
      content,
      blog_image,
      {
        accounts: {
          authority: authority,
          userProfile: userProfile,
          blogAccount: blogAccount,
          systemProgram: web3.SystemProgram.programId
        }
      }
    );

    // print the signature of transaction
    console.log("Your transaction signature", tx);
  }); 

  it("update existing blog", async () => {
    const title = "some new title";
    const content = "some new content";
    const blog_image = "some new image url";

    // address of authority
    let authority = provider.wallet.publicKey;

    // address of userProfile
    let [blogAccount, blogAccount_b] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("BLOG_STATE"), authority.toBuffer(), new anchor.BN(0).toBuffer()], 
      program.programId,
    );

    // call the program to update the blog at index 0
    const tx = await program.rpc.updateBlog(
      0,
      title,
      content,
      blog_image,
      {
        accounts: {
          authority: authority,
          blogAccount: blogAccount,
          systemProgram: web3.SystemProgram.programId
        }
      }
    );

    // print the signature of transaction
    console.log("Your transaction signature", tx);
  }); 

  it("delete existing blog", async () => {
    
    // address of authority
    let authority = provider.wallet.publicKey;

    // address of userProfile
    let [userProfile, userProfile_b] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("USER_STATE"), authority.toBuffer()], 
      program.programId,
    );

    // address of userProfile
    let [blogAccount, blogAccount_b] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("BLOG_STATE"), authority.toBuffer(), new anchor.BN(0).toBuffer()], 
      program.programId,
    );

    // call the program to delete the blog at index 0
    const tx = await program.rpc.deleteBlog(
      0,
      {
        accounts: {
          authority: authority,
          userProfile: userProfile,
          blogAccount: blogAccount,
          systemProgram: web3.SystemProgram.programId
        }
      }
    );

    // print the signature of transaction
    console.log("Your transaction signature", tx);
  }); 
});
