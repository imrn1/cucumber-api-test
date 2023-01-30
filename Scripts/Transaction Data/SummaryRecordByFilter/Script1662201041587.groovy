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


WS.sendRequestAndVerify(findTestObject('Postman/TransactionData/SummaryRecordByFilter', [
		"id": "${id}",
		"ext_transaction_id": "${ext_transaction_id}",
		"tenant_id": "${tenant_id}",
		"wallet_id": "${wallet_id}",
		"process_level_status_id": "${process_level_status_id}",
		"transaction_type_code": "${transaction_type_code}",
		"to_wallet_id": "${to_wallet_id}",
		"currency_code": "${currency_code}",
		"from_account_number": "${from_account_number}",
		"from_account_type_Id": "${from_account_type_Id}",
		"from_account_Id": "${from_account_Id}",
		"from_wallet_number": "${from_wallet_number}",
		"to_account_type_Id": "${to_account_type_Id}",
		"to_account_Id": "${to_account_Id}",
		"to_account_number": "${to_account_number}",
		"to_wallet_number": "${to_wallet_number}",
		"description": "${description}",
		"transaction_direction": "${transaction_direction}",
		"start_date": "${start_date}",
		"end_date": "${end_date}",
		"min_amount": "${min_amount}",
		"max_amount": "${max_amount}",
		"media_identifier": "${media_identifier}",
		"provider_id": "${provider_id}",
		"page_size": "${page_size}",
		"page_index": "${page_index}",
		"order_column": "${order_column}",
		"order_by": "${order_by}"
	]))














