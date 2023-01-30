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
import java.time.format.DateTimeFormatter as DateTimeFormatter
import java.time.LocalDateTime as LocalDateTime




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


/******************************************************************/

	def accountNumber = "$account_number"
	if (!(isNullOrEmpty("$account_number")) && ("$account_number" != '123')) {
		//println("sender account number rnd degil, null/empty degil : "+"$sender_account_number")
	   
		WebUI.callTestCase(findTestCase('_Custom Cases/set sender_account--sender_wallet--receiver_wallet'),
			[
				('name_for_sender_account_number') : accountNumber,
				('name_for_sender_wallet_number') : '',
				('name_for_receiver_wallet_number') : ''
			])
	
		accountNumber = GlobalVariable.default_sender_account_number
	}
		
	
	def walletNumber = "$wallet_number"
	if (!(isNullOrEmpty("$wallet_number")) && ("$wallet_number" != '123')) {
		//println("sender account number rnd degil, null/empty degil : "+"$sender_account_number")
	   
		WebUI.callTestCase(findTestCase('_Custom Cases/set sender_account--sender_wallet--receiver_wallet'),
			[
				('name_for_sender_account_number') : '',
				('name_for_sender_wallet_number') : '',
				('name_for_receiver_wallet_number') : walletNumber
			])
	
		walletNumber = GlobalVariable.default_receiver_wallet_number
	}
		
	
/******************************************************************/
	
WebUI.callTestCase(findTestCase('_Custom Cases/Wallet Info'),
	[
		('account_number') : accountNumber,
		('wallet_number') : walletNumber,
		('description') : "$process_description",
		('beforeTransaction') : 1,
		('serviceName') : 'TopupCreditCard'
	])

def TopupCreditCardResponse = WS.sendRequestAndVerify(findTestObject('Postman/Transaction/TopupCreditCard', 
	[
		('ext_transaction_id') : transaction_id
        , ('credit_card_post_type_id') : "$credit_card_post_type_id", ('amount') : "$amount", ('currency_code') : "$currency_code"
        , ('account_number') : accountNumber, 
		('wallet_number') : walletNumber,
		('hash_key') : "$hash_key"
	]))




//  serviceName, processDescription,  expected_status, expected_code,  expected_message
CustomKeywords.'writeFile.WriteFile.writeExel'('TopupCreditCard', "$process_description", "$expected_status", "$expected_code",
	"$expected_message", fileName="TopupCreditCard","$expected_ResponseStatusCode")


WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"] )


WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'TopupCreditCard'] )





WebUI.callTestCase(findTestCase('_Custom Cases/Wallet Info'),
	[
		('account_number') : accountNumber,
		('wallet_number') : walletNumber,
		('description') : "$process_description",
		('beforeTransaction') : 2,
		('serviceName') : 'TopupCreditCard'
	])
 









