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



/**********************                ************************/
def AccountNumber = "$account_number"
if (!(isNullOrEmpty("$account_number")) && ("$account_number" != '123')) {
	//println("sender account number rnd degil, null/empty degil : "+"$sender_account_number")
   
	WebUI.callTestCase(findTestCase('_Custom Cases/set sender_account--sender_wallet--receiver_wallet'),
		[
			('name_for_sender_account_number') : AccountNumber,
			('name_for_sender_wallet_number') : '',
			('name_for_receiver_wallet_number') : ''
		])

	AccountNumber = GlobalVariable.default_sender_account_number

	println("-->"+"$account_number"+" : "+AccountNumber )
	
	}

/*****************************************************/




def BusinessAccountSubmitKYCFormResponse = WS.sendRequestAndVerify(findTestObject('Postman/Account/BusinessAccountSubmitKYCForm', 
        [
			('account_number') : AccountNumber,
			 ('first_name') : "$first_name", ('last_name') : "$last_name",
			 ('national_id') : "$national_id"
            , ('birth_year') : "$birth_year", ('birth_month') : "$birth_month", ('birth_day') : "$birth_day", 
			('national_document_type_id') : "$national_document_type_id"
            , ('country_code') : "$country_code",
			
			"sector_id": "$sector_id",
			"address": "$address",
			"province_id": "$province_id",
			"city": "$city"
		]))



//  serviceName, processDescription,  expected_status, expected_code,  expected_message
CustomKeywords.'writeFile.WriteFile.writeExel'('BusinessAccountSubmitKYCForm',"$process_description","$expected_status","$expected_code",
	"$expected_message","BusinessAccountSubmitKYCForm","$expected_ResponseStatusCode")
 


WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"]  )

WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"]  )

WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'BusinessAccountSubmitKYCForm'] )

