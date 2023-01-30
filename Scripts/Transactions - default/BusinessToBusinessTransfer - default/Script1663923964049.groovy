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


def transaction_id = "$ext_transaction_id"

if (isNullOrEmpty(transaction_id)) {
	WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)
	transaction_id = GlobalVariable.uniqe_id
}
else
	transaction_id = "";

	

/**********************                ************************/
def senderAccountNumber = "$sender_account_number"
if (!(isNullOrEmpty("$sender_account_number")) && ("$sender_account_number" != '123')) {
    //println("sender account number rnd degil, null/empty degil : "+"$sender_account_number")
   
	WebUI.callTestCase(findTestCase('_Custom Cases/set sender_account--sender_wallet--receiver_wallet'), 
		[
			('name_for_sender_account_number') : senderAccountNumber,
			('name_for_sender_wallet_number') : '', 
			('name_for_receiver_wallet_number') : ''
		])

    senderAccountNumber = GlobalVariable.default_sender_account_number
}
	
			
		
def senderWalletNumber = "$sender_wallet_number"
if(!isNullOrEmpty("$sender_wallet_number") && "$sender_wallet_number" != "123") {
		//println("sender wallet number rnd degil, null/empty degil : "+"$sender_wallet_number")
		
	WebUI.callTestCase(findTestCase('_Custom Cases/set sender_account--sender_wallet--receiver_wallet'),
		[
			('name_for_sender_account_number') : '',
			('name_for_sender_wallet_number') : senderWalletNumber,
			('name_for_receiver_wallet_number') : ''
		])
	senderWalletNumber = GlobalVariable.default_sender_wallet_number
}
	
	
def receiverAccountNumber = ""
def receiverWalletNumber = "$receiver_wallet_number"
if(!isNullOrEmpty("$receiver_wallet_number") && "$receiver_wallet_number" != "123") {
		//println("receiver wallet number rnd degil, null/empty degil : "+"$receiver_wallet_number")
		
	WebUI.callTestCase(findTestCase('_Custom Cases/set sender_account--sender_wallet--receiver_wallet'),
		[
			('name_for_sender_account_number') : '',
			('name_for_sender_wallet_number') : '',
			('name_for_receiver_wallet_number') : receiverWalletNumber
		])
	receiverWalletNumber = GlobalVariable.default_receiver_wallet_number
	receiverAccountNumber = GlobalVariable.account_number
}	
/**********************                ************************/
	
WebUI.callTestCase(findTestCase('_Custom Cases/Wallet Info'),
	[
		('account_number') : senderAccountNumber,
		('wallet_number') : senderWalletNumber,
		('description') : "$process_description"+ "\n sender wallet info",
		('beforeTransaction') : 1,
		('serviceName') : 'Payment'
	])

WebUI.callTestCase(findTestCase('_Custom Cases/Wallet Info'),
	[
		('account_number') : receiverAccountNumber,
		('wallet_number') : receiverWalletNumber,
		('description') : "$process_description"+ "\n receiver wallet info",
		('beforeTransaction') : 1,
		('serviceName') : 'Payment'
	])

def BusinessToBusinessTransferResponse = WS.sendRequestAndVerify(findTestObject('Postman/Transaction/BusinessToBusinessTransfer',[
		"ext_transaction_id": transaction_id,
		"sender_account_number": senderAccountNumber,
		"sender_wallet_number": senderWalletNumber,
	  
		"amount": "${amount}",
	  
		"currency_code": "${currency_code}",
		"description": "${description}",
	   
		"receiver_wallet_number": receiverWalletNumber,
		"hash_key": "${hash_key}"
	]))


CustomKeywords.'writeFile.WriteFile.writeExel'('BusinessToBusinessTransfer',"$process_description","$expected_status","$expected_code",
	"$expected_message", fileName="BusinessToBusinessTransfer","$expected_ResponseStatusCode")
 

WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"] )


WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'BusinessToBusinessTransfer'] )



WebUI.callTestCase(findTestCase('_Custom Cases/Wallet Info'),
	[
		('account_number') : senderAccountNumber,
		('wallet_number') : senderWalletNumber,
		('description') : "$process_description"+ "\n sender wallet info",
		('beforeTransaction') : 2,
		('serviceName') : 'Payment'
	])

WebUI.callTestCase(findTestCase('_Custom Cases/Wallet Info'),
	[
		('account_number') : receiverAccountNumber,
		('wallet_number') : receiverWalletNumber,
		('description') : "$process_description"+ "\n receiver wallet info",
		('beforeTransaction') : 2,
		('serviceName') : 'Payment'
	])








