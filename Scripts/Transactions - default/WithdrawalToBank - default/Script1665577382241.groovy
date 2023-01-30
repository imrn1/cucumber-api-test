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
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

static boolean isNullOrEmpty(String str) {
	return (str == null) || str.allWhitespace
}



/*********************************************/
def receiverAccountNumber = "$receiver_account_number"

//println("receiver_account_number : "+"$receiver_account_number")

if (!(isNullOrEmpty("$receiver_account_number")) && ("$receiver_account_number" != '123')) {
	//println("sender account number rnd degil, null/empty degil : "+"$sender_account_number")
	WebUI.callTestCase(findTestCase('_Custom Cases/set sender_account--sender_wallet--receiver_wallet'),
		[
			('name_for_sender_account_number') : receiverAccountNumber,
			('name_for_sender_wallet_number') : '',
			('name_for_receiver_wallet_number') : ''
		])

	receiverAccountNumber = GlobalVariable.default_sender_account_number
}


def nationalID = "$receiver_national_id"
println("--> tc no : "+nationalID)

if(isNullOrEmpty("$receiver_national_id") ) {
	nationalID = GlobalVariable.national_id   
	println("--> tc no (global v.) : "+nationalID)
}else if(nationalID == "null") {
	nationalID = ""
	println("--> tc no : "+nationalID)
}else if(nationalID != '123') {
	WebUI.callTestCase(findTestCase('_Custom Cases/set sender_account--sender_wallet--receiver_wallet'),
		[
			('name_for_sender_account_number') : nationalID,
			('name_for_sender_wallet_number') : '',
			('name_for_receiver_wallet_number') : ''
		])
	nationalID = GlobalVariable.national_id
}


def receiverWalletNumber = "$receiver_wallet_number"
if (!(isNullOrEmpty("$receiver_wallet_number")) && ("$receiver_wallet_number" != '123')) {
	//println("sender account number rnd degil, null/empty degil : "+"$sender_account_number")
	WebUI.callTestCase(findTestCase('_Custom Cases/set sender_account--sender_wallet--receiver_wallet'),
		[
			('name_for_sender_account_number') : '',
			('name_for_sender_wallet_number') : '',
			('name_for_receiver_wallet_number') : receiverWalletNumber
		])

	receiverWalletNumber = GlobalVariable.default_receiver_wallet_number
}



def extTransactionID = "$ext_transaction_id"
if(extTransactionID == 'null') {
	extTransactionID = ''
	println("extTransactionID = null >> "+extTransactionID)
}else if(isNullOrEmpty("$ext_transaction_id")) {
	WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)
	extTransactionID = GlobalVariable.uniqe_id
	println("extTransactionID = null or empty >> "+extTransactionID)
}else if(extTransactionID == 'same') {
	extTransactionID = GlobalVariable.uniqe_id
	println("extTransactionID = same >> "+extTransactionID)
}

/*******************************************************/

def WithdrawalToBankResponse = WS.sendRequestAndVerify(findTestObject('Postman/Transaction/WithdrawalToBank',
	[
		 "ext_transaction_id": extTransactionID,
		  "receiver_account_number" : receiverAccountNumber,
		  "receiver_wallet_number" : receiverWalletNumber,
		  "receiver_national_id":nationalID,
		  "receiver_account_holder_name":"${receiver_account_holder_name}",
		  "receiver_iban":"${receiver_iban}",
		  "description":"${description}",
		  "amount":"${amount}",
		  "currency_code":"${currency_code}",
		  "is_save_bank_account":"${is_save_bank_account}",
		  "bank_account_name" : "${bank_account_name}",
		  "hash_key": "${hash_key}"
	]))



CustomKeywords.'writeFile.WriteFile.writeExel'('WithdrawalToBank',"$process_description","$expected_status","$expected_code",
	"$expected_message",'WithdrawalToBank')
  
WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'WithdrawalToBank'] )






