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

println("Register personal")

def personal_list = new ArrayList()

for (int i = 0; i < Integer.valueOf("$count"); i++) {
	
	 WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE) 
	 def RegisterPersonalAccountResponse = WS.sendRequestAndVerify(findTestObject('Postman/Account/Register_PersonalAccount', 
            [('currency_code') : "$currency_code", 
				('user_phone_country_code') : "$user_phone_country_code", 
				('user_phone_number') :  GlobalVariable.uniqe_id.toString().substring(4)
                , ('use_iban') : "$use_iban"
			])) 
	 
	 // dosyaya yazılmalı mı kontrolü
	 if("$writeDoc" == "true" ) {
		 CustomKeywords.'writeFile.WriteFile.writeExel'('Register_PersonalAccount', '', '0', '', '', fileName = "$docName")
	 }
	 	  
	 if(WS.getResponseStatusCode(RegisterPersonalAccountResponse) == 200) {  // kayıt başarılı 
		
		 if(Objects.isNull(GlobalVariable.personal_list)) {
			 GlobalVariable.personal_list = [[
				 ('account_number') : WS.getElementPropertyValue(RegisterPersonalAccountResponse, "payload.account_number"),
				 ('wallet_number') : WS.getElementPropertyValue(RegisterPersonalAccountResponse, "payload.wallet_number"),
				 ("kyc_level") : WS.getElementPropertyValue(RegisterPersonalAccountResponse, "payload.kyc_level_code")
			 ] ]
		 }else {
			 GlobalVariable.personal_list.add(
			 [
				 ('account_number') : WS.getElementPropertyValue(RegisterPersonalAccountResponse, "payload.account_number"),
				 ('wallet_number') : WS.getElementPropertyValue(RegisterPersonalAccountResponse, "payload.wallet_number"),
				 ("kyc_level") : WS.getElementPropertyValue(RegisterPersonalAccountResponse, "payload.kyc_level_code")
			 ] )
		 }
		 
		
	 }
	 else
		 WS.verifyResponseStatusCode(RegisterPersonalAccountResponse, 200)  // hata olduğunu göstermeli
	 
}


println("(ArrayList)(GlobalVariable.personal_list).size : " + (GlobalVariable.personal_list).size())
for(def item : GlobalVariable.personal_list) {
	println('item["account_number"] : '+item["account_number"])
	println('item["wallet_number"] : '+item["wallet_number"])
	println('item["kyc_level"] : '+item["kyc_level"])
}







