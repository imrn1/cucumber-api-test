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



 
 def SetCardAsPassiveResponse = WS.sendRequestAndVerify(findTestObject('Postman/CardPackage/SetCardAsPassive', [('account_number') : "$account_number"
			 , ('wallet_number') : "$wallet_number", ('card_no') : "$card_no"]))
 
 if (WS.getElementPropertyValue(SetCardAsPassiveResponse, 'status') == 0) {
	 // kart aktif edilirse işlemlerde kullanılmak üzere listeye eklenir
	 if (GlobalVariable.passiveCardNo == null) {
		 GlobalVariable.passiveCardNo = []
	 }
	 
	 GlobalVariable.passiveCardNo.add("$card_no")
 
	 // account_number ve wallet_number degerleri de listeye eklenir
	 if (GlobalVariable.sender_account_number == null) {
		 GlobalVariable.sender_account_number = []
	 }
	 
	 GlobalVariable.sender_account_number.add("$account_number")
 
	 if (GlobalVariable.sender_wallet_number == null) {
		 GlobalVariable.sender_wallet_number = []
	 }
	 
	 GlobalVariable.sender_wallet_number.add("$wallet_number")
 }
 
 CustomKeywords.'writeFile.WriteFile.writeExel'('SetCardAsPassive', "$process_description", "$expected_status", "$expected_code",
	 "$expected_message", 'SetCardAsPassive',"$expected_ResponseStatusCode")
 
 WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"])
 
 WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"])
 
 WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"])
 
 WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"])
 
 WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'SetCardAsPassive'])
 
 
 



