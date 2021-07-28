<template>
<tr>
  <td>{{ account.token_mint_addr }}</td>
  <td>{{ account.token_assoc_addr }}</td>
  <td>{{ account.token_balance }}</td>
  <td><button class="bg-solana-purple text-white italic p-1" @click=transferTokens>send</button></td>
</tr>
</template>

<script>
import axios from "axios";

export default {
  props: {
    account: Object,
    sol_pubkey: String,
    to_pubkey: String,
    amount: Number,
  },
  methods: {
    async transferTokens() {
      await axios.post("http://localhost:5000/transfer_tokens", {
        mint_addr: this.account.token_mint_addr,
        from_pubkey: this.sol_pubkey,
        to_pubkey: this.to_pubkey,
        amount: this.amount,
      })
    }
  }
}
</script>

<style scoped>
th {
  @apply p-1;
  text-align: left;
}
td {
  @apply p-1;
  text-align: left;
}
</style>
