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



def personalList = new ArrayList() 
if(Integer.valueOf("$getAll") == Integer.valueOf('1')) { // tüm hesaplar alınır GlobalVariable.personal_list e atanır
		

	def PersonalListByFilterResponse =  WS.sendRequestAndVerify(findTestObject('Postman/Account/PersonalListByFilter',[ 
			"order_column":"Id",
			"page_size": 0,
			"page_index": 0,
			"id": 0,
			"tenant_id": 0
		]))
	
	
	// dosyaya yazılmalı mı kontrolü
	if("$writeDoc" == "true" ) {
		CustomKeywords.'writeFile.WriteFile.writeExel'('PersonalListByFilter', '', '', '', '', fileName = "$docName")
	}
	
	
	ArrayList results = WS.getElementPropertyValue(PersonalListByFilterResponse, "payload.results")
	
	// println("results size : "+results.size() )
	// println("results class : "+results.getClass() )
	
	
	GlobalVariable.personal_list_size =  Integer.valueOf("$list_size") // gerek yok gibi
	
	if(Integer.valueOf("$list_size") != 0 ) {
		for(int i = 0 ; i<Integer.valueOf("$list_size") ; i++) {
			//println('results["account_number"] : '+ results[i]["account_number"])
			personalList.add( 
				[
					 ('account_number') : results[i]["account_number"], 
					 ('kyc_level') : results[i]["user_kyc_info"]["kyc_level"],
					 ('national_id') : results[i]["user_kyc_info"]["national_id"],
					 ('first_name') : results[i]["user_kyc_info"]["first_name"],
					 ('last_name') : results[i]["user_kyc_info"]["last_name"]	 
				 ] )
		}
	}else {
		for(int i = 0 ; i <results.size() ; i++) {
			// println('results["account_number"] : '+results[i]["account_number"])
			personalList.add(
				 [
					('account_number') : results[i]["account_number"], 
					('kyc_level') : results[i]["user_kyc_info"]["kyc_level"] ,
					('national_id') : results[i]["user_kyc_info"]["national_id"],
					('first_name') : results[i]["user_kyc_info"]["first_name"],
					('last_name') : results[i]["user_kyc_info"]["last_name"]
				])
		}
	}
	
	
	println("get personal list")
	 for(def item : personalList) {
		 println('item["f.name"] : '+item["first_name"])
		 println('item["l.name"] : '+item["last_name"])
		 
		 println('item["account_number"] : '+item["account_number"])
		 println('item["wallet_number"] : '+item["wallet_number"])
		 println('item["kyc_level"] : '+item["kyc_level"])
		 println('item["national_id"] : '+item["national_id"])
		 
	 }
	
	GlobalVariable.personal_list = personalList;
		
}else {  // "$personal_list" içerisindeki  	
	
	
	for(def listitem : GlobalVariable.personal_list) {
		
 
		// dönen cevaptki results 1 deger dönmeli
		def PersonalListByFilterResponse =  WS.sendRequestAndVerify(findTestObject('Postman/Account/PersonalListByFilter',[
			"order_column":"Id",
			"page_size": 0,
			"page_index": 0,
			"id": 0,
			"tenant_id": 0,
			"account_number" : listitem["account_number"]
		]))
		
		
		// dosyaya yazılmalı mı kontrolü
		if("$writeDoc" == "true" ) {
			CustomKeywords.'writeFile.WriteFile.writeExel'('PersonalListByFilter', '', '', '', '', fileName = "$docName")
		}
		
		def results = WS.getElementPropertyValue(PersonalListByFilterResponse, "payload.results")
		personalList.add(
			[
				 ('account_number') : results[0]["account_number"],
				 ('kyc_level') : results[0]["user_kyc_info"]["kyc_level"],
				 ('national_id') : results[0]["user_kyc_info"]["national_id"],
				 ('first_name') : results[0]["user_kyc_info"]["first_name"],
				 ('last_name') : results[0]["user_kyc_info"]["last_name"]
			 ] )
		
		println("get personal list")
		for(def item1 : personalList) {
			println('item["f.name"] : '+item1["first_name"])
			println('item["l.name"] : '+item1["last_name"])
			
			println('item["account_number"] : '+item1["account_number"])
			println('item["wallet_number"] : '+item1["wallet_number"])
			println('item["kyc_level"] : '+item1["kyc_level"])
			println('item["national_id"] : '+item1["national_id"])
			
		}
	   
	   GlobalVariable.personal_list = personalList;
	   
	   
		
	}
	
}


 