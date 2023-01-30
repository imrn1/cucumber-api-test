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


static boolean isNullOrEmpty(String str) { return (str == null || str.allWhitespace) }


def PersonalListByFilterResponse = WS.sendRequestAndVerify(findTestObject('Postman/Account/PersonalListByFilter', [('id') : "$id", ('tenant_id') : "$tenant_id"
            , ('account_id') : "$account_id", ('account_number') : "$account_number", ('first_name') : "$first_name", ('last_name') : "$last_name"
            , ('end_date') : "$end_date", ('start_date') : "$start_date", ('access_level_status_id') : "$access_level_status_id"
            , ('kyc_level') : "$kyc_level", ('phone_number') : "$phone_number", ('email') : "$email", ('phone_country_code') : "$phone_country_code"
            , ('national_id') : "$national_id", ('page_size') : "$page_size", ('page_index') : "$page_index", ('order_column') : "$order_column"
            , ('order_by') : "$order_by"]))

 
if(!isNullOrEmpty("$id") ) {
	// payload kontrolü yapılmalı
	try {
		 	def dataList = WS.getElementPropertyValue(PersonalListByFilterResponse,"payload.results")
			 
			for(def data: dataList) {
				println("account_number : ": + data.account_number)
			}
		}
   catch(Exception ex){
	   println("pesonal list by filter catch block")
   }

}




CustomKeywords.'writeFile.WriteFile.writeExel'('PersonalListByFilter',"$process_description","$expected_status","$expected_code",
	"$expected_message",'PersonalListByFilter',"$expected_ResponseStatusCode")
 
 

WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'PersonalListByFilter'])


