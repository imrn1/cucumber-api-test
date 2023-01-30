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

def SetCardLimitAndRestrictionresponse = WS.sendRequestAndVerify(findTestObject('Postman/CardPackage/SetCardLimitAndRestriction',[
		"account_number": "${account_number}",
		"wallet_number": "${wallet_number}",
		"card_no": "${card_no}",
		"auth_ecom": "${auth_ecom}",
		"auth_moto": "${auth_moto}",
		"auth_contactless": "${auth_contactless}",
		"auth_int": "${auth_int}",
		"set_limit": "${set_limit}",
		"daily_max_amount": "${daily_max_amount}",
		"weekly_max_amount": "${weekly_max_amount}",
		"monthly_max_amount": "${monthly_max_amount}",
		"yearly_max_amount": "${yearly_max_amount}"
	]))


CustomKeywords.'writeFile.WriteFile.writeExel'('SetCardLimitAndRestriction',"$process_description","$expected_status","$expected_code","$expected_message")


WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'SetCardLimitAndRestriction'] )


