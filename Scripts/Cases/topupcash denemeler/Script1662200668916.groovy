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

WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)

def TopupCashResponse = WS.sendRequestAndVerify(findTestObject('Postman/Transaction/TopupCash', [('tenant_code') : "$tenant_code"
            , ('sender_name') : "$sender_name", ('sender_iban') : "$sender_iban", ('receiver_bank_code') : "$receiver_bank_code"
            , ('receiver_bank_name') : "$receiver_bank_name", ('receiver_iban') : "$receiver_iban", ('amount') : "$amount"
            , ('currency_code') : "$currency_code", ('description') : "$description", ('regex_definition_result_data') : "$regex_definition_result_data"
            , ('bank_transaction_date') : "$bank_transaction_date", ('bank_receipt_no') : GlobalVariable.uniqe_id, ('national_id_or_tax_no') : "$national_id_or_tax_no"
            , ('wallet_number') : "$wallet_number"]))


CustomKeywords.'writeFile.WriteFile.writeExel'('TopupCash', "$process_description", "to_wallet_number : "+"$wallet_number", "to_account_number : "+"$regex_definition_result_data",
	"national_id_or_tax_no"+"$national_id_or_tax_no", fileName = 'TopupCashdenemeler', "$expected_ResponseStatusCode")


def recordResponse = WS.sendRequestAndVerify(findTestObject('Postman/TransactionData/SummaryRecordByFilter',
	[
		"id": 0,
		"ext_transaction_id": GlobalVariable.uniqe_id,
		"tenant_id": 0,
		"wallet_id": 0,
		"process_level_status_id": "0",
		"transaction_type_code": "",
		"to_wallet_id": 0,
		"currency_code": "",
		"from_account_number": "",
		"from_account_type_Id": "0",
		"from_account_Id": 0,
		"from_wallet_number": "",
		"to_account_type_Id": "0",
		"to_account_Id": 0,
		"to_account_number": "",
		"to_wallet_number": "",
		"description": "",
		"transaction_direction": "",
		"start_date": "",
		"end_date": "",
		"min_amount": "0",
		"max_amount": "0",
		"media_identifier": "",
		"provider_id": "",
		"page_size": 0,
		"page_index": 0,
		"order_column": "Id",
		"order_by": "asc"
	]))


def to_account_number = WS.getElementPropertyValue(recordResponse, "payload.results[0].to_account_number")
def to_wallet_number = WS.getElementPropertyValue(recordResponse, "payload.results[0].to_wallet_number")


	 CustomKeywords.'writeFile.WriteFile.writeExel'('SummaryRecordByFilter', "$process_description", "to_wallet_number : "+to_wallet_number, "to_account_number : "+to_account_number,
		 "", fileName = 'TopupCashdenemeler', "$expected_ResponseStatusCode")
	 



