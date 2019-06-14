console.log("ok")

var Long = require("long");
var amount= new Long(0x12345678, 0, true);
var fee = new Long(0xedcba988, 0xffffffff, true); // amount's 2 complement
var sum = amount.add(fee);
var mybalnace = new Long(0x1, 0, true);
if (sum < mybalnace) {
    console.log("valid tx")
}
else {
    console.log("not valid");
}
console.log("amount="+amount.toString());
console.log("fee="+fee.toString());
console.log("sum="+sum.toString());
console.log("my balance="+ mybalnace.toString());