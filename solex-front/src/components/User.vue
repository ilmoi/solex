<template>
  <div class="bg-solana-verylightgreen m-5 p-5 w-full">
    <table class="w-full table-fixed">
      <tr>
        <th class="w-1/3">Token</th>
        <th class="w-1/3">Address</th>
        <th class="w-1/6">Balance</th>
        <th class="w-1/6">Send</th>
      </tr>
      <tr>
        <td>sol</td>
        <td>{{ account.sol_pubkey }}</td>
        <td>{{ account.sol_balance }}</td>
        <td>
          <button
              class="bg-solana-purple text-white italic p-1"
              @click="transferLamports"
          >send
          </button>
        </td>
      </tr>
      <Row
          v-for="a in account.token_balances"
          :account="a"
          :sol_pubkey="account.sol_pubkey"
          :to_pubkey="to_pubkey"
          :amount="amount"/>
    </table>
  </div>
</template>

<script>
import Row from "@/components/Row";
import axios from 'axios'

export default {
  components: {Row},
  props: {
    account: Object,
    to_pubkey: String,
    amount: Number,
  },
  methods: {
    async transferLamports() {
      await axios.post("http://localhost:5000/transfer", {
        from_pubkey: this.account.sol_pubkey,
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