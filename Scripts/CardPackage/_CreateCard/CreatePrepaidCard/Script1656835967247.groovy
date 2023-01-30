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

WS.sendRequestAndVerify(findTestObject('Postman/CardPackage/CreatePrepaidCard', [
	  "account_number": "${account_number}",
	  "wallet_number": "${wallet_number}",
	  "account_type": "${account_type}",
	  "currency_code": "${currency_code}",
	  "product_code": "${product_code}",	 
	  "city": "${city}",
	  "city_code": "${city_code}",
	  "address_type": "${address_type}",
	  "country_code": "${country_code}",
	  "district": "${district}",
	  "Address1": "${Address1}",
	  "Address2": "${Address2}",
	  "Address3": "${Address3}",
	  "Address4":"${Address4}" ,
	  "address_code": "${address_code}",
	  "zip_code": "${zip_code}"
	]))

 
CustomKeywords.'writeFile.WriteFile.writeExel'('CreatePrepaidCard',"$process_description","$expected_status","$expected_code","$expected_message",
	fileName ="CreatePrepaidCard","$expected_ResponseStatusCode")
 
 
WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'CreatePrepaidCard'] )











