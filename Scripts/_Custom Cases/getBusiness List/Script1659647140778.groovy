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


def businessList = new ArrayList()
if(Integer.valueOf("$getAll") == Integer.valueOf('1')) { // tüm hesaplar alınır GlobalVariable.personal_list e atanır
		

	def BusinessListByFilterResponse =  WS.sendRequestAndVerify(findTestObject('Postman/Account/BusinessListByFilter',[
			"order_column":"Id",
			"page_size": 0,
			"page_index": 0,
			"id": 0,
			"tenant_id": 0
		]))
	
	
	// dosyaya yazılmalı mı kontrolü
	if("$writeDoc" == "true" ) {
		CustomKeywords.'writeFile.WriteFile.writeExel'('BusinessListByFilter', '', '', '', '', fileName = "$docName")
	}
	
	
	ArrayList results = WS.getElementPropertyValue(BusinessListByFilterResponse, "payload.results")
	
	// println("results size : "+results.size() )
	// println("results class : "+results.getClass() )
	
	
	GlobalVariable.business_list_size =  Integer.valueOf("$list_size") // gerek yok gibi
	
	if(Integer.valueOf("$list_size") != 0 ) {
		for(int i = 0 ; i<Integer.valueOf("$list_size") ; i++) {
			//println('results["account_number"] : '+ results[i]["account_number"])
			businessList.add(
				[
					 ('account_number') : results[i]["account_number"], 
					 ('name') : results[i]["name"]
				 ] )
		}
	}else {
		for(int i = 0 ; i <results.size() ; i++) {
			// println('results["account_number"] : '+results[i]["account_number"])
			businessList.add(
				 [
					('account_number') : results[i]["account_number"], 
					 ('name') : results[i]["name"]
				])
		}
	}
	
	
	println("get business list")
	 for(def item : businessList) {
		 println('item["name"] : '+item["name"])
		 println('item["account_number"] : '+item["account_number"])
	 }
	
	GlobalVariable.business_list = businessList;
		
}else {  // "$business_list" içerisindeki
	
	
	for(def listitem : GlobalVariable.business_list) {
		
 
		// dönen cevaptki results 1 deger dönmeli
		def BusinessListByFilterResponse =  WS.sendRequestAndVerify(findTestObject('Postman/Account/BusinessListByFilter',[
			"order_column":"Id",
			"page_size": 0,
			"page_index": 0,
			"id": 0,
			"tenant_id": 0,
			"account_number" : listitem["account_number"]
		]))
		
		
		// dosyaya yazılmalı mı kontrolü
		if("$writeDoc" == "true" ) {
			CustomKeywords.'writeFile.WriteFile.writeExel'('BusinessListByFilter', '', '', '', '', fileName = "$docName")
		}
		
		def results = WS.getElementPropertyValue(BusinessListByFilterResponse, "payload.results")
		businessList.add(
			[
				 ('account_number') : results[0]["account_number"],
				 ('name') : results[0]["name"]
			 ] )
		
		println("get business list")
		for(def item1 : businessList) {
			println('item["name"] : '+item1["name"])
			println('item["account_number"] : '+item1["account_number"])
		}
	   
	   GlobalVariable.business_list = businessList;
	   
	   
		
	}
	
}


 





