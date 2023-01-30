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


def user_phone_number = "$user_phone_number"

if (isNullOrEmpty(user_phone_number)) {
	WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)
	user_phone_number = GlobalVariable.uniqe_id.toString().substring(4)
}
else if(user_phone_number == "invalid") {
	WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)
	user_phone_number = GlobalVariable.uniqe_id.toString().substring(7)
}
else
	user_phone_number = "";
 

def RegisterPersonalAccountResponse = WS.sendRequestAndVerify(findTestObject('Postman/Account/Register_PersonalAccount', 
        [('account_number') : "$account_number", ('currency_code') : "$currency_code",
			 ('alias') : "$alias", ('user_number') : "$user_number"
            , ('user_first_name') : "$user_first_name", ('user_last_name') : "$user_last_name", 
			('user_phone_country_code') : "$user_phone_country_code"
            ,('user_phone_number') : "$user_phone_number",
			"use_iban":"${use_iban}",
			 ('user_email') : "$user_email", ('first_name') : "$first_name"
            , ('last_name') : "$last_name", ('email') : "$email", ('address_line1') : "$address_line1", ('address_line2') : "$address_line2"
            , ('zip_postal_code') : "$zip_postal_code", ('phone_number') : "$phone_number", ('state_province_code') : "$state_province_code"
            , ('country_code') : "$country_code"]))

  
CustomKeywords.'writeFile.WriteFile.writeExel'('Register_PersonalAccount',"$process_description","$expected_status","$expected_code",
	"$expected_message",'Register_PersonalAccount',"$expected_ResponseStatusCode")
 
 
WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"])

 
WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'RegisterPersonalAccount'])



