import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import customKeyword.CustomMethods as Custom
import customKeyword.Enums as Enums
import customKeyword.Account as Account
import customKeyword.Wallet as Wallet

import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import customKeyword.Verify as Verify

import com.kms.katalon.core.testobject.ResponseObject
import org.json.JSONObject;

import com.google.gson.Gson;




/*
def x = "NİD".toLowerCase(new Locale("en"))
println(x)
*/


//WebUI.callTestCase(findTestCase('_Custom Cases/_Token alma'), [('token') : GlobalVariable.token], FailureHandling.STOP_ON_FAILURE)


  
Account account = new Account()

def x;
x = Custom.isServiceCall("$variable") 

println("test case : \n"  + "x : "+x )

 

if(x.getClass().equals(Account)) {
	
	/*
	println("if(x.getClass().equals(Account)) =>")
	 
	println("acc number:"+x.AccountNumber)
	println("acc kyc:"+x.kycInfo.toString())
	println("acc wallets :"+x.Wallets)
*/
	
	
	println("gson : \n"+ new Gson().toJson(x).toString())
	
	 
	
	}
	

	
	
	









//Custom.getTCNOList()

/*
Account account = new Account()
Account ReceiverAccount = new Account()
//account.getAccountByAccountNumber("1005586009") // işletme hesabı - nb : 135,30


account.getAccountByAccountNumber("0000",Enums.AccountType.personal) //  hem işletme hem bireysel
ReceiverAccount.getAccountByAccountNumber("2835885722") // işletme hesabı


Wallet beforeWallet = account.setBeforeWallet()
Wallet RbeforeWallet = ReceiverAccount.setBeforeWallet()



// payment işlemi yaptıralımm...
Custom.sendRequest("/v1/Transaction/Payment", """{
  "ext_transaction_id": "2022522812512025",
  "sender_account_number": "0000",
  "sender_wallet_number": "412492408",
  "currency_code": "TRY",
  "amount": "1",
  "receiver_wallet_number": "1685099926"
}""")



Wallet afterWallet =  account.setAfterWallet()
Wallet RafterWallet = ReceiverAccount.setAfterWallet()


println("""
before sender wallet (${beforeWallet.Number}) bakiye:
cash avb: ${beforeWallet.CashBalance.available_balance}
cash unavb : ${beforeWallet.CashBalance.unavailable_balance}
""")

println("""
after sender wallet (${afterWallet.Number}) bakiye:
cash avb: ${afterWallet.CashBalance.available_balance}
cash unavb : ${afterWallet.CashBalance.unavailable_balance}
""")

println("*******************************")

println("""
before receiver wallet (${RbeforeWallet.Number}) bakiye:
cash avb: ${RbeforeWallet.CashBalance.available_balance}
cash unavb : ${RbeforeWallet.CashBalance.unavailable_balance}
""")		

println("""
after receiver wallet (${RafterWallet.Number}) bakiye:
cash avb: ${RafterWallet.CashBalance.available_balance}
cash unavb : ${RafterWallet.CashBalance.unavailable_balance}
""")


*/
/*
account.getAccountByAccountNumber("0000",Enums.AccountType.business)

println(account.AccountNumber)
println(account.AccountType)
*/
/*
account.getAccountByAccountNumber("0000",Enums.AccountType.personal)

println(account.AccountNumber)
println(account.AccountType)
*/

//println("kyc level : "+ account.kycInfo.kycLevel)


/*
def x = Custom.GenerateUniqeNumber()
println(x)

x = Custom.GenerateUniqeNumber()
println(x)

x = Custom.GenerateUniqeNumber()
println(x)


*/

 




/*
ResponseObject WalletListResponse =  WS.sendRequestAndVerify(findTestObject('Postman/Wallet/ListByFilter', [
			"account_number": "\"9211978932\"",
			"page_size": 10,
			"page_index": 1,
			"order_column": "\"Id\"",
			"order_by": "\"asc\""
		]))

ArrayList walletList = WS.getElementPropertyValue(WalletListResponse, "payload.results")


Dictionary accountTypes = new Hashtable();
for(int i = 0 ; i<walletList.size() ; i++){
	accountTypes.put(walletList[i]["account_type"], 1)		
}


for(String keys : accountTypes.keys()){
	println("keys : "+keys)
}
println("keys.length : "+ accountTypes.keys().size())

*/
















/*
def response = WS.sendRequestAndVerify(findTestObject('Postman/Account/Get Personal Account', [('account_number') : '8586939478']))

WebUI.callTestCase(findTestCase('_Verify Cases/Verify Element Property Value'), 
	[
		('value_name') : 'payload.updated_date_utc', 
		('variable') : "$expected_status"
	])

*/


/*

Account account = new Account();

account.getAccountByAccountNumber("8586939478")

println(account.AccountNumber)
println(account.AccountType)

println("kyc level : "+ account.kycInfo.kycLevel)  

account.VerifyKyc("$kycLevelasd")

 */

/*
def response = Custom.sendRequest("/v1/Account/GetPersonalAccount", "$body")

JSONObject json = new JSONObject(response.getBodyContent().getText());
println("test case -> response body\n"+json.toString(4));
*/

/*
String x = """{
    "code": "100",
    "payload": {
        "tenant_id": "5",
        "last_failed_login_date_utc": null,
        "account_number": "İMRAN0101",
        "contact_address_zip_postal_code": null,
    """


println(x.length())
*/


/*

def accountnumber = Account.FirstAccountNumberFromPersonalList()
println("FirstAccountNumberFromPersonalList() -> account Number : " +accountnumber )

*/


/*
x = Custom.isServiceCall("$accountNumber")
println("deger: "+"$accountNumber" + " account no : "+ x)


x = Custom.isServiceCall("$walletNumber")
println("deger: "+"$walletNumber" + " cüzdan no : "+ x)


x = Custom.isServiceCall("$NationlID")
println("deger: "+"$NationlID" + " NationlID : "+ x)



x = Custom.isServiceCall("$kycLevel")
println("deger: "+"$kycLevel" + " kycLevel : "+ x)

*/

//x = Custom.isServiceCall("$verifyKyc")
//println("deger: "+"$verifyKyc" + " verifyKyc : "+ x)




/*

String[] params = []


params = "a,b,c,d".split(",")

println(params.toString())

println("drop : " + params.drop(1))

println(params.toString())

*/


/*
x = Custom.isServiceCall("$expected_status")
println("deger: "+"$expected_status" + " x : "+ x)

x = Custom.isServiceCall("$kycLevel")
println("deger: "+"$kycLevel" + " x : "+ x)

x = Custom.isServiceCall("$name")
println("deger: "+"$name" + " x : "+ x)

 
*/




/*
x = Custom.isServiceCall("$user_phone_country_code")
println("x : "+ x)
*/




/*


v1 = Custom.GenerateUniqeNumber()

wait(1000)

v2 = Custom.GenerateUniqeNumber()

wait(1000)

v3 = Custom.GenerateUniqeNumber()

wait(1000)

v4 = Custom.GenerateUniqeNumber()


println(v1)
println(v2)
println(v3)
println(v4)

*/






















