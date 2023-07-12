// use mo_types::{Principal, HashMap, Nat, Text};

// actor Token {
//     owner: Principal = Principal::from_text("5u4bd-g3mtt-av5mg-6wwnx-ceghq-zkxrq-xipwb-kjxlt-gx7nr-5tkgh-iae"),
//     total_supply: Nat = 1000000000,
//     symbol: Text = "DUNYA".to_string(),
//     balances: HashMap<Principal, Nat> = HashMap::new(1, Principal::equal, Principal::hash),
// }

// impl Token {
//     pub async fn balance_of(&self, who: Principal) -> Nat {
//         match self.balances.get(&who) {
//             None => 0,
//             Some(result) => *result,
//         }
//     }
// }

// fn main() {
//     let token = Token {
//         owner: Principal::from_text("5u4bd-g3mtt-av5mg-6wwnx-ceghq-zkxrq-xipwb-kjxlt-gx7nr-5tkgh-iae"),
//         total_supply: 1000000000,
//         symbol: "DUNYA".to_string(),
//         balances: HashMap::new(1, Principal::equal, Principal::hash),
//     };

//     token.balances.put(token.owner.clone(), token.total_supply);
// }
