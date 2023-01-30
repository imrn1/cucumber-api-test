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



def BusinessListByFilterResponse = WS.sendRequestAndVerify(findTestObject('Postman/Account/BusinessListByFilter',
	[
		"tanant_id": "${tanant_id}",
		"account_type_id":"${account_type_id}" ,
		"account_id":"${account_id}",
		"account_number": "${account_number}",
		"page_size": "${page_size}",
		"page_index": "${page_index}",
		"order_column": "${order_column}",
		"order_by": "${order_by}",
		"name": "${name}",
		"start_date": "${start_date}",
		"end_date": "${end_date}",
		"access_status_id": "${access_status_id}",
		"email": "${email}",
		"phone_number": "${phone_number}",
		"country_code": "${country_code}"
	]))

/*
CustomKeywords.'writeFile.WriteFile.writeExel'('BusinessListByFilter',"$process_description","$expected_status","$expected_code",
	"$expected_message","BusinessListByFilter","$expected_ResponseStatusCode")
 */


if(!isNullOrEmpty(verify)) {
	// verify == check_business_type ise response payload -> results -> business_type =? "${account_type_id}" 
	if("$verify" == "check_business_type") {
		def results = WS.getElementPropertyValue(BusinessListByFilterResponse, "payload.results")
		
		def i = 0
		for( def item : results ) {
			if(i == 2)
				break;
			println(item["account_number"])
			println(item["business_type"])
			i++
		}
		
		
	}
	
}




WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"] )
 
WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'BusinessListByFilter'])

