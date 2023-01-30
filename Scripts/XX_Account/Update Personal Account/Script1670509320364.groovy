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

def UpdatePersonalAccountResponse = WS.sendRequestAndVerify(findTestObject('Postman/Account/UpdatePersonalAccount', [('account_id') : "$account_id"
            , ('id') : "$id", ('tenant_id') : "$tenant_id", ('account_number') : "$account_number", ('alias') : "$alias"
            , ('kyc_level') : "$kyc_level", ('owner_user_id') : "$owner_user_id", ('contact_address_contact_first_name') : "$contact_address_contact_first_name"
            , ('contact_address_contact_last_name') : "$contact_address_contact_last_name", ('contact_address_contact_email') : "$contact_address_contact_email"
            , ('contact_address_contact_phone') : "$contact_address_contact_phone", ('contact_address_address_line1') : "$contact_address_address_line1"
            , ('contact_address_address_line2') : "$contact_address_address_line2", ('contact_address_zip_postal_code') : "$contact_address_zip_postal_code"
            , ('contact_address_state_province_code') : "$contact_address_state_province_code", ('contact_address_country_code') : "$contact_address_country_code"
            , ('access_level_status_id') : "$access_level_status_id", ('created_date_utc') : "$created_date_utc", ('updated_date_utc') : "$updated_date_utc"]))

 
//  serviceName, processDescription,  expected_status, expected_code,  expected_message
CustomKeywords.'writeFile.WriteFile.writeExel'('UpdatePersonalAccount',"$process_description","$expected_status","$expected_code",
	"$expected_message","UpdatePersonalAccount","$expected_ResponseStatusCode")


WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"])

WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : "UpdatePersonalAccount"], FailureHandling.STOP_ON_FAILURE)

