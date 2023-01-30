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

def receipt_no = "$bank_receipt_no"

if (isNullOrEmpty(receipt_no)) {
    WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)

    receipt_no = GlobalVariable.uniqe_id
} else {
    receipt_no = ''
}

//println("bank_receipt_no : "+receipt_no)
/**********************                ************************/
def descriptionAccountNumber = "$description"

if (!(isNullOrEmpty("$description")) && ("$description" != '123')) {
    //println("sender account number rnd degil, null/empty degil : "+"$sender_account_number")
    WebUI.callTestCase(findTestCase('_Custom Cases/set sender_account--sender_wallet--receiver_wallet'), [('name_for_sender_account_number') : descriptionAccountNumber
            , ('name_for_sender_wallet_number') : '', ('name_for_receiver_wallet_number') : ''])

    descriptionAccountNumber = GlobalVariable.default_sender_account_number
}

def regexDefinitionResultDataAccountNumber = "$regex_definition_result_data"

if (!(isNullOrEmpty("$regex_definition_result_data")) && ("$regex_definition_result_data" != '123')) {
    //println("sender account number rnd degil, null/empty degil : "+"$sender_account_number")
    WebUI.callTestCase(findTestCase('_Custom Cases/set sender_account--sender_wallet--receiver_wallet'), [('name_for_sender_account_number') : regexDefinitionResultDataAccountNumber
            , ('name_for_sender_wallet_number') : '', ('name_for_receiver_wallet_number') : ''])

    regexDefinitionResultDataAccountNumber = GlobalVariable.default_sender_account_number
}

def nationalID = "$national_id_or_tax_no"

println('--> tc no : ' + nationalID)

if (isNullOrEmpty("$national_id_or_tax_no")) {
    nationalID = GlobalVariable.national_id // --> RegexDefinitionRD ile bulunan hesaba ait tc no tutulur

    println('--> tc no (global v.) : ' + nationalID)
} else if (nationalID == 'null') {
    nationalID = ''

    println('--> tc no : ' + nationalID)
}

def WalletNumber = "$wallet_number"

if (!(isNullOrEmpty("$wallet_number")) && ("$wallet_number" != '123')) {
    //println("sender account number rnd degil, null/empty degil : "+"$sender_account_number")
    WebUI.callTestCase(findTestCase('_Custom Cases/set sender_account--sender_wallet--receiver_wallet'), [('name_for_sender_account_number') : ''
            , ('name_for_sender_wallet_number') : '', ('name_for_receiver_wallet_number') : WalletNumber])

    WalletNumber = GlobalVariable.default_receiver_wallet_number
}

/*************************************************************/

WebUI.callTestCase(findTestCase('_Custom Cases/Wallet Info'), 
	[
		('account_number') : regexDefinitionResultDataAccountNumber, 
		('wallet_number') : WalletNumber, 
		('description') : "$process_description",
	    ('beforeTransaction') : 1, 
		('serviceName') : 'TopupCash'
	])

def TopupCashResponse = WS.sendRequestAndVerify(findTestObject('Postman/Transaction/TopupCash', [('tenant_code') : "$tenant_code"
            , ('sender_name') : "$sender_name", ('sender_iban') : "$sender_iban", ('receiver_bank_code') : "$receiver_bank_code"
            , ('receiver_bank_name') : "$receiver_bank_name", ('receiver_iban') : "$receiver_iban", ('amount') : "$amount"
            , ('currency_code') : "$currency_code", ('bank_transaction_date') : "$bank_transaction_date", ('bank_receipt_no') : receipt_no
            , ('description') : descriptionAccountNumber // account number
            , ('regex_definition_result_data') : regexDefinitionResultDataAccountNumber // account number
            , ('national_id_or_tax_no') : nationalID, ('wallet_number') : WalletNumber]))


if (WS.getElementPropertyValue(TopupCashResponse, 'status') == 0) {
	GlobalVariable.tx_correlation_id = WS.getElementPropertyValue(TopupCashResponse, 'tx_correlation_id')
}

//  serviceName, processDescription,  expected_status, expected_code,  expected_message
CustomKeywords.'writeFile.WriteFile.writeExel'('TopupCash', "$process_description", "$expected_status", "$expected_code",
	"$expected_message", fileName = "$file_name", "$expected_ResponseStatusCode")

WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"])

WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"])

WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"])

WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"])

WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'TopupCash'])



WebUI.callTestCase(findTestCase('_Custom Cases/Wallet Info'),
	[
		('account_number') : regexDefinitionResultDataAccountNumber,
		('wallet_number') : WalletNumber,
		('description') : "$process_description",
		('beforeTransaction') : 2,
		('serviceName') : 'TopupCash'
	])



static boolean isNullOrEmpty(String str) {
    return (str == null) || str.allWhitespace
}

