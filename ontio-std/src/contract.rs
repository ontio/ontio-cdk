pub mod ont {
    use super::super::base58;
    use super::super::types::{Address, U128};
    const ONT_CONTRACT_ADDRESS: Address = base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhUMqNMV");

    pub struct State {
        pub from: Address,
        pub to: Address,
        pub amount: U128,
    }

    pub fn transfer(transfer: &[State]) -> bool {
        super::util::transfer_inner(&ONT_CONTRACT_ADDRESS, transfer)
    }

    pub fn approve(from: &Address, to: &Address, amount: U128) -> bool {
        super::util::approve_inner(&ONT_CONTRACT_ADDRESS, from, to, amount)
    }

    pub fn balance_of(address: &Address) -> U128 {
        super::util::balance_of_inner(&ONT_CONTRACT_ADDRESS, &address)
    }

    pub fn allowance(from: &Address, to: &Address) -> U128 {
        super::util::allowance_inner(&ONT_CONTRACT_ADDRESS, from, to)
    }

    pub fn transfer_from(sender: &Address, from: &Address, to: &Address, amount: U128) -> bool {
        super::util::transfer_from_inner(&ONT_CONTRACT_ADDRESS, sender, from, to, amount)
    }
}

pub mod ong {
    use super::super::base58;
    use super::super::types::{Address, U128};
    const ONG_CONTRACT_ADDRESS: Address = base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhfRZMHJ");
    pub fn transfer(transfer: &[super::ont::State]) -> bool {
        super::util::transfer_inner(&ONG_CONTRACT_ADDRESS, transfer)
    }
    pub fn balance_of(address: &Address) -> U128 {
        super::util::balance_of_inner(&ONG_CONTRACT_ADDRESS, &address)
    }
    pub fn approve(from: &Address, to: &Address, amount: U128) -> bool {
        super::util::approve_inner(&ONG_CONTRACT_ADDRESS, from, to, amount)
    }
    pub fn allowance(from: &Address, to: &Address) -> U128 {
        super::util::allowance_inner(&ONG_CONTRACT_ADDRESS, from, to)
    }
    pub fn transfer_from(sender: &Address, from: &Address, to: &Address, amount: U128) -> bool {
        super::util::transfer_from_inner(&ONG_CONTRACT_ADDRESS, sender, from, to, amount)
    }
}

pub(crate) mod util {
    use super::super::abi::Sink;
    use super::super::runtime;
    use super::super::types::{to_neo_bytes, u128_from_neo_bytes, Address, U128};
    use core::convert::TryInto;

    const VERSION: u8 = 0;
    pub(crate) fn transfer_inner(
        contract_address: &Address, transfer: &[super::ont::State],
    ) -> bool {
        let mut sink = Sink::new(16);
        sink.write_native_varuint(transfer.len() as u64);

        for state in transfer.iter() {
            sink.write_native_address(&state.from);
            sink.write_native_address(&state.to);
            sink.write(to_neo_bytes(state.amount));
        }
        let mut sink_param = Sink::new(16);
        sink_param.write(VERSION);
        sink_param.write("transfer");
        sink_param.write(sink.bytes());
        let res = runtime::call_contract(contract_address, sink_param.bytes());
        if let Some(data) = res {
            if data.len() != 0 {
                return true;
            }
        }
        false
    }

    pub(crate) fn approve_inner(
        contract_address: &Address, from: &Address, to: &Address, amount: U128,
    ) -> bool {
        let mut sink = Sink::new(16);
        sink.write_native_address(from);
        sink.write_native_address(to);
        sink.write(to_neo_bytes(amount));
        let mut sink_param = Sink::new(16);
        sink_param.write(VERSION);
        sink_param.write("approve");
        sink_param.write(sink.bytes());
        let res = runtime::call_contract(contract_address, sink_param.bytes());
        if let Some(data) = res {
            if data.len() != 0 {
                return true;
            }
        }
        false
    }

    pub(crate) fn transfer_from_inner(
        contract_address: &Address, sender: &Address, from: &Address, to: &Address, amount: U128,
    ) -> bool {
        let mut sink = Sink::new(16);
        sink.write_native_address(sender);
        sink.write_native_address(from);
        sink.write_native_address(to);
        sink.write(to_neo_bytes(amount));
        let mut sink_param = Sink::new(16);
        sink_param.write(VERSION);
        sink_param.write("transferFrom");
        sink_param.write(sink.bytes());
        let res = runtime::call_contract(contract_address, sink_param.bytes());
        if let Some(data) = res {
            if data.len() != 0 {
                return true;
            }
        }
        false
    }

    pub(crate) fn allowance_inner(
        contract_address: &Address, from: &Address, to: &Address,
    ) -> U128 {
        let mut sink = Sink::new(0);
        sink.write_native_address(from);
        sink.write_native_address(to);
        let mut sink_param = Sink::new(0);
        sink_param.write(VERSION);
        sink_param.write("allowance");
        sink_param.write(sink.bytes());
        let res = runtime::call_contract(contract_address, sink_param.bytes());
        if let Some(data) = res {
            if data.len() != 0 {
                return u128_from_neo_bytes(data);
                // todo: impl from neobytes
            }
        }
        0
    }
    pub(crate) fn balance_of_inner(contract_address: &Address, address: &Address) -> U128 {
        let mut sink = Sink::new(0);
        sink.write_native_address(address);
        let mut sink_param = Sink::new(0);
        sink_param.write(VERSION);
        sink_param.write("balanceOf");
        sink_param.write(sink.bytes());
        let res = runtime::call_contract(contract_address, sink_param.bytes());
        if let Some(data) = res {
            if data.len() != 0 {
                return u128_from_neo_bytes(data);
                //                return U128::from_le_bytes(data.as_slice().try_into().unwrap());
                // todo: impl from neobytes
            }
        }
        0
    }
}
