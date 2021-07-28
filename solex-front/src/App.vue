<template>
  <div class="flex flex-col items-center justify-center">
    <Button class="bg-solana-purple text-white p-5 m-5 text-2xl italic" @click="createNewAccount">NEW ACCOUNT</Button>
    <div class="m-5">
      <label for="dest" class="p-1">Destination: </label>
      <input id="dest" type="text" class="p-1 bg-solana-verylightpurple" v-model="to_pubkey">
      <label for="amnt" class="p-1">Amount: </label>
      <input id="amnt" type="number" class="p-1 w-12 bg-solana-verylightpurple" v-model.number="amount">
    </div>
    <User v-for="a in accounts" :key="a.sol_pubkey" :account="a" :amount="amount" :to_pubkey="to_pubkey"/>
  </div>
</template>

<script>
import axios from 'axios';
import User from "@/components/User";

export default {
  components: {User},
  data() {
    return {
      accounts: [],
      amount: 1,
      to_pubkey: "12KKNE2g3Tajwe3ogAaoG63swV8GXpaxrxJyBSeMs648",
    }
  },
  methods: {
    async createNewAccount() {
      await axios.get("http://localhost:5000/create");
      await this.refreshAccounts();
    },
    async refreshAccounts() {
      this.accounts = (await axios.get("http://localhost:5000/accounts")).data;
    }
  },
  async created() {
    await this.refreshAccounts();
  }
}
</script>

<style scoped>

</style>