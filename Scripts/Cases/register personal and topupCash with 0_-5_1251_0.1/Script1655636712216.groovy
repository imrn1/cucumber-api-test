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


if (GlobalVariable.RegisterPersonalAccountResponse == null) {
	WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)
	
    def RegisterPersonalAccountResponse = WS.sendRequestAndVerify(findTestObject('Postman/Account/Register_PersonalAccount', [
		('state_province_code') : "$state_province_code"
            , ('user_phone_country_code') : "$user_phone_country_code", 
			('user_phone_number') : "535"+GlobalVariable.uniqe_id.toString().substring(7), 
			('currency_code') : "$currency_code"]))

    GlobalVariable.RegisterPersonalAccountResponse = GlobalVariable.responseObject
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$RegisterPersonalAccount_expected_status"])
	
	//String serviceName="" , String processDescription="", String expected_status="" ,String expected_code="", String expected_message="", String fileName =""
	CustomKeywords.'writeFile.WriteFile.writeExel'('Register_PersonalAccount', "", "","","",fileName ="register personal and topupcash")


} else {
	
	WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)
	
    def TopupCashResponse = WS.sendRequestAndVerify(findTestObject('Postman/Transaction/TopupCash', [('tenant_code') : "$tenant_code"
                , ('sender_name') : "$sender_name", ('sender_iban') : "$sender_iban", ('receiver_bank_code') : "$receiver_bank_code"
                , ('receiver_bank_name') : "$receiver_bank_name", ('receiver_iban') : "$receiver_iban", ('amount') : "$amount"
                , ('currency_code') : "$currency_code", 
				('description') : WS.getElementPropertyValue(GlobalVariable.RegisterPersonalAccountResponse, "payload.account_number"), 
				('regex_definition_result_data') : WS.getElementPropertyValue(GlobalVariable.RegisterPersonalAccountResponse, "payload.account_number")
                , ('bank_transaction_date') : "$bank_transaction_date", 
				('bank_receipt_no') : GlobalVariable.uniqe_id, 
				('national_id_or_tax_no') : "$national_id_or_tax_no"
                , ('wallet_number') : WS.getElementPropertyValue(GlobalVariable.RegisterPersonalAccountResponse, "payload.wallet_number")
	]))

	//String serviceName="" , String processDescription="", String expected_status="" ,String expected_code="", String expected_message="", String fileName ="" 
    CustomKeywords.'writeFile.WriteFile.writeExel'('TopupCash', "$process_description", "$TopupCashExpectedStatus","","",fileName ="register personal and topupcash")

    WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$TopupCashExpectedStatus"])
}

