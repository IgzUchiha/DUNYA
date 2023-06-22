// import Principal "mo:base/Principal";
// import HashMap "mo:base/HashMap";
// import Prelude "mo:base/Prelude";



// actor Token {
//     var owner : Principal = Principal.fromText("5u4bd-g3mtt-av5mg-6wwnx-ceghq-zkxrq-xipwb-kjxlt-gx7nr-5tkgh-iae");
//     var totalSupply : Nat = 1000000000;
//     var symbol : Text = "DUNYA";
//     var balances = HashMap.HashMap<Principal, Nat>(1, Principal.equal, Principal.hash);

//     balances.put(owner, totalSupply);

//     public query func balanceOf (who: Principal) : async Nat {
//         let balance : Nat = switch (balances.get(who)){
//             case null 0;
//             case (?result) result;
//         };
        
//         return balance;
    
//     }
 


// };