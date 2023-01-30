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

	

def BusinessToBusinessTransferResponse = WS.sendRequestAndVerify(findTestObject('Postman/Transaction/BusinessToBusinessTransfer',[
		"ext_transaction_id": transaction_id,
		"sender_account_number": "${sender_account_number}",
		"sender_wallet_number": "${sender_wallet_number}",
	  
		"amount": "${amount}",
	  
		"currency_code": "${currency_code}",
		"description": "${description}",
	   
		"receiver_wallet_number": "${receiver_wallet_number}",
		"hash_key": "${hash_key}"
	]))


CustomKeywords.'writeFile.WriteFile.writeExel'('BusinessToBusinessTransfer',"$process_description","$expected_status","$expected_code",
	"$expected_message", fileName="BusinessToBusinessTransfer","$expected_ResponseStatusCode")
 

WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"] )


WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'BusinessToBusinessTransfer'] )







