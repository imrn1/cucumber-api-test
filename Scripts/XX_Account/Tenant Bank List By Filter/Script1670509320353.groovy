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

WS.sendRequestAndVerify(findTestObject('Postman/Account/TenantBankListByFilter',[
	"tenant_id": "${tenant_id}",
	"draw": "${draw}",
	 
		"data": "${data}",
		"name": "${name}",
		"searchable": "${searchable}",
		"orderable": "${orderable}",
		 
			"value": "${value}",
			"is_regex": "${is_regex}",
	 
		"column": "${column}",
		"dir": "${dir}",
	 
	"start": "${start}",
	"length": "${length}",
	
		"value": "${value1}",
		"is_regex": "${is_regex1}",

  
  "page_size": "${page_size}",
  "page_index": "${page_index}",
  "order_column": "${order_column}",
  "order_by": "${order_by}"
	
	]))


//  serviceName, processDescription,  expected_status, expected_code,  expected_message
CustomKeywords.'writeFile.WriteFile.writeExel'('TenantBankListByFilter',"$process_description","$expected_status","$expected_code",
	"$expected_message","TenantBankListByFilter","$expected_ResponseStatusCode")
 
 
WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"])

WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'TenantBankListByFilter'])


