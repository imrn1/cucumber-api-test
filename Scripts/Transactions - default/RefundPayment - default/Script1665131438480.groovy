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

 
def transaction_id = "$ext_transaction_id"


def correlation_id = "${tx_correlation_id}"
	

if (isNullOrEmpty(correlation_id)) {
	correlation_id = ""
	
	WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)
	transaction_id = GlobalVariable.uniqe_id
	 
	RefundPaymentResponse = WS.sendRequestAndVerify(findTestObject('Postman/Transaction/RefundPayment',[
		"ext_transaction_id":transaction_id,
		"tx_correlation_id":correlation_id,
		"currency_code":"${currency_code}",
		"refund_amount":"$refund_amount",
		"description":"${description}"
	]))
  

	CustomKeywords.'writeFile.WriteFile.writeExel'('RefundPayment',"$process_description","$expected_status","$expected_code",
		"$expected_message", fileName="RefundPayment","$expected_ResponseStatusCode")
	 
	WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"] )
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"] )
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"] )
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"] )
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'RefundPayment'] )
	
}
else {
	 
	if(correlation_id== "loop")
	{
		correlation_id = GlobalVariable.tx_correlation_id
				
		if( "$refund_amount" == "100" )
		{
			WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)
			transaction_id = GlobalVariable.uniqe_id
			 
			RefundPaymentResponse = WS.sendRequestAndVerify(findTestObject('Postman/Transaction/RefundPayment',[
				"ext_transaction_id":transaction_id,
				"tx_correlation_id":correlation_id[0],
				"currency_code":"${currency_code}",
				"refund_amount":"$refund_amount",
				"description":"${description}"
			]))
		  
		
			CustomKeywords.'writeFile.WriteFile.writeExel'('RefundPayment',"$process_description","$expected_status","$expected_code",
				"$expected_message", fileName="RefundPayment","$expected_ResponseStatusCode")
			 
			WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"] )
			
			WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"] )
			
			WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"] )
			
			WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"] )
			
			WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'RefundPayment'] )
			
		}
		else 
	    {
			 for(int i=0; i< GlobalVariable.tx_correlation_id.size(); i++)
			 {
				 WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)
				 transaction_id = GlobalVariable.uniqe_id
				 
				   def refund_amount = GlobalVariable.refund_amount[i]
				   
				   /*
				   if(Float.parseFloat(refund_amount) == 0) { // amount : 0 olan ödeme iadesinden beklenen sonuçlar bu alanda setlendi
					   def expected_status = "2"
					   def expected_code = "null"
					   def expected_message = "Sistem Hatası!"
					   def expected_ResponseStatusCode = "200"
				   } else {
					   expected_status = "0"
					   expected_code = "100"
					   expected_message = "İşlem başarılı"
					   expected_ResponseStatusCode = "200"
				   }  */
				   
				   RefundPaymentResponse = WS.sendRequestAndVerify(findTestObject('Postman/Transaction/RefundPayment',[
					   "ext_transaction_id":transaction_id,
					   "tx_correlation_id":correlation_id[i],
					   "currency_code":"${currency_code}",
					   "refund_amount":refund_amount,
					   "description":"${description}"
				   ]))
				 
				   CustomKeywords.'writeFile.WriteFile.writeExel'('RefundPayment',"$process_description","$expected_status","$expected_code",
					   "$expected_message", fileName="RefundPayment","$expected_ResponseStatusCode")
					
				   WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"] )
				   
				   WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"] )
				   
				   WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"] )
				   
				   WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"] )
				   
				   WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'RefundPayment'] )	
				   
			 }  
		}	 
	}
	else if(correlation_id == 'refunded') {
		correlation_id = GlobalVariable.tx_correlation_id[0]
		if (transaction_id == 'null') {
			transaction_id = ''
		} else {
			WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)

			transaction_id = GlobalVariable.uniqe_id
		}
		
		RefundPaymentResponse = WS.sendRequestAndVerify(findTestObject('Postman/Transaction/RefundPayment', [
			('ext_transaction_id') : transaction_id
			, ('tx_correlation_id') : correlation_id, ('currency_code') : "$currency_code",
			('refund_amount') : GlobalVariable.refund_amount[0]
			, ('description') : "$description"]))

		CustomKeywords.'writeFile.WriteFile.writeExel'('RefundPayment', "$process_description", "$expected_status", "$expected_code",
			"$expected_message", fileName = 'RefundPayment',"$expected_ResponseStatusCode")
		
		WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"])
		
		WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"])
		
		WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"])
		
		WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"])
		
		WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'RefundPayment'])

		
	}
	else if(correlation_id == 'partial_refund') {
		correlation_id = GlobalVariable.tx_correlation_id_for_partial_refund
		
		WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)
		transaction_id = GlobalVariable.uniqe_id
		
		RefundPaymentResponse = WS.sendRequestAndVerify(findTestObject('Postman/Transaction/RefundPayment', [
			('ext_transaction_id') : transaction_id,
			('tx_correlation_id') : correlation_id,
			('currency_code') : "$currency_code",
			('refund_amount') : "$refund_amount",
			('description') : "$description"]))

		CustomKeywords.'writeFile.WriteFile.writeExel'('RefundPayment', "$process_description", "$expected_status", "$expected_code",
			"$expected_message", fileName = 'RefundPayment',"$expected_ResponseStatusCode")
		
		WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"])
		WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"])
		WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"])
		WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"])
		WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'RefundPayment'])

	}
	else 
	{
		if(transaction_id == "null")
			transaction_id = "";
		else {
			WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)
			transaction_id = GlobalVariable.uniqe_id
		}
		RefundPaymentResponse = WS.sendRequestAndVerify(findTestObject('Postman/Transaction/RefundPayment',[
			"ext_transaction_id":transaction_id,
			"tx_correlation_id":correlation_id,
			"currency_code":"${currency_code}",
			"refund_amount":"$refund_amount",
			"description":"${description}"
		]))
	  
	
		CustomKeywords.'writeFile.WriteFile.writeExel'('RefundPayment',"$process_description","$expected_status","$expected_code",
			"$expected_message", fileName="RefundPayment","$expected_ResponseStatusCode")
		 
		WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"] )
		
		WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"] )
		
		WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"] )
		
		WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"] )
		
		WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'RefundPayment'] )
		
	}
	 
}
	
	 
	
	 