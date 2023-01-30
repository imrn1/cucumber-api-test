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

def phone_number = "$user_phone_number"
def user_email = "$user_email"
def tax_number = "$tax_number"


if("$user_phone_number" == "G") {
	WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)
	phone_number = GlobalVariable.uniqe_id.toString().substring(4)
	GlobalVariable.phone_number = phone_number
}
if("$user_phone_number" == "same") {
	phone_number = GlobalVariable.phone_number
}

if("$user_phone_number" == "null") {
	phone_number = ""
}

// --------------------------------------------------- // 
if("$user_email" == "G") {
	WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)
	user_email = "test"+GlobalVariable.uniqe_id.toString().substring(4)+"@mail.com"
	GlobalVariable.user_email = user_email
}
if("$user_email" == "same") {
	user_email = GlobalVariable.user_email
}

if("$user_email" == "null") {
	user_email = ""
}

// --------------------------------------------------- //

if("$tax_number" == "G") {
	WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)
	tax_number = GlobalVariable.uniqe_id.toString().substring(3)
	GlobalVariable.tax_number = tax_number
}
if("$tax_number" == "same") {
	tax_number = GlobalVariable.tax_number
}

if("$tax_number" == "null") {
	tax_number = ""
}


/*
println("tax_number : "+tax_number)

println("user_email : "+user_email)

println("phone_number : "+phone_number)

*/


def RegisterBusinessAccountResponse = WS.sendRequestAndVerify(findTestObject('Postman/Account/Register_BusinessAccount', 
        [('account_number') : "$account_number", ('currency_code') : "$currency_code", 
			('user_email') : user_email,
			 ('tax_number') : tax_number
            , ('business_type') : "$business_type",
			 ('name') : "$name", ('alias') : "$alias", ('user_number') : "$user_number"
            , ('user_first_name') : "$user_first_name", ('user_last_name') : "$user_last_name", 
			('user_phone_country_code') : "$user_phone_country_code"
            , ('user_phone_number') : phone_number, 
			('tax_office') : "$tax_office",
			('use_iban') : "$use_iban", 
			('first_name') : "$first_name"
            , ('last_name') : "$last_name", 
			('email') : "$email", 
			('address_line1') : "$address_line1", ('address_line2') : "$address_line2"
            , ('zip_postal_code') : "$zip_postal_code", ('phone_number') : "$phone_number", ('state_province_code') : "$state_province_code"
            , ('country_code') : "$country_code"]))

CustomKeywords.'writeFile.WriteFile.writeExel'('Register_BusinessAccount', "$process_description", "$expected_status", "$expected_code", 
    "$expected_message", 'Register_BusinessAccount', "$expected_ResponseStatusCode")

WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"])

WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"])

WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"])

WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"])

WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'RegisterBusinessAccount'])



