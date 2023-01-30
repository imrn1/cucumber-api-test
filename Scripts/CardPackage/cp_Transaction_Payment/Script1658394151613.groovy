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

if (isNullOrEmpty(transaction_id)) {
	WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)
	transaction_id = GlobalVariable.uniqe_id
	
	def cp_Transaction_PaymentResponse = WS.sendRequestAndVerify(findTestObject('Postman/CardPackage/cp_Transaction_Payment', [
				('ext_transaction_id') : transaction_id,
				('sender_account_number') : "$sender_account_number", ('sender_wallet_number') : "$sender_wallet_number"
				, ('currency_code') : "$currency_code",
				 ('amount') : "$amount",
				 ('description') : "$description", ('receiver_wallet_id') : "$receiver_wallet_id"
				, ('media_identifier') : "$media_identifier",  // dosyadan alınıyor kart no 
				 ('source_type') : "$source_type", ('channel_type') : "$channel_type"
				, ('hash_key') : "$hash_key", ('terminal_no') : "$terminal_no"]))
	
	
	
	if(WS.getElementPropertyValue(cp_Transaction_PaymentResponse, "status") == 0) {
		// ödeme başarılı
		def correlation_id = WS.getElementPropertyValue(cp_Transaction_PaymentResponse, "payload.transaction_id")
		println("correlation_id : "+correlation_id)
		println("payment_amount : " + "$amount" )
		
		if(GlobalVariable.tx_correlation_id == null) {
			GlobalVariable.tx_correlation_id = []
		}
		GlobalVariable.tx_correlation_id.add(correlation_id)
		
		
		if(GlobalVariable.payment_amount == null) {
			GlobalVariable.payment_amount = []
		}
		GlobalVariable.payment_amount.add("$amount")
	}
	
	 
	 
	CustomKeywords.'writeFile.WriteFile.writeExel'('Payment',"$process_description","$expected_status","$expected_code",
		"$expected_message", fileName="cp_Transaction_Payment","$expected_ResponseStatusCode")
	 
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"] )
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"] )
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"] )
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"] )
	
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'cp_Transaction_Payment'] )
	
}
else {
	if("$ext_transaction_id" == "null") 
	{
		transaction_id = "";
		
		def cp_Transaction_PaymentResponse = WS.sendRequestAndVerify(findTestObject('Postman/CardPackage/cp_Transaction_Payment', [
					('ext_transaction_id') : transaction_id,
					('sender_account_number') : "$sender_account_number", ('sender_wallet_number') : "$sender_wallet_number"
					, ('currency_code') : "$currency_code",
					 ('amount') : "$amount",
					 ('description') : "$description", ('receiver_wallet_id') : "$receiver_wallet_id"
					, ('media_identifier') : "$media_identifier", // dosyadan alınıyor kart no 
					 ('source_type') : "$source_type", ('channel_type') : "$channel_type"
					, ('hash_key') : "$hash_key", ('terminal_no') : "$terminal_no"]))
		
		
		
		if(WS.getElementPropertyValue(cp_Transaction_PaymentResponse, "status") == 0) {
			// ödeme başarılı
			def correlation_id = WS.getElementPropertyValue(cp_Transaction_PaymentResponse, "payload.transaction_id")
			println("correlation_id : "+correlation_id)
			println("payment_amount : " + "$amount" )
			
			if(GlobalVariable.tx_correlation_id == null) {
				GlobalVariable.tx_correlation_id = []
			}
			GlobalVariable.tx_correlation_id.add(correlation_id)
			
			
			if(GlobalVariable.payment_amount == null) {
				GlobalVariable.payment_amount = []
			}
			GlobalVariable.payment_amount.add("$amount")
		}
		
		 
		 
		CustomKeywords.'writeFile.WriteFile.writeExel'('Payment',"$process_description","$expected_status","$expected_code",
			"$expected_message", fileName="cp_Transaction_Payment","$expected_ResponseStatusCode")
		 
		
		WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"] )
		
		WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"] )
		
		WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"] )
		
		WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"] )
		
		
		WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'cp_Transaction_Payment'] )
		
			
		
	}
	else {
		
		switch("$ext_transaction_id") {
			case "activeCards":
			
			println(" activeCards switch içerisinde")
				for(int i=0; i< GlobalVariable.activeCardNo.size(); i++)
				{
					WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)
					transaction_id = GlobalVariable.uniqe_id
					
					
					
					def GetClearCardNoResponse = WS.sendRequestAndVerify(findTestObject('Postman/CardPackage/GetClearCardNo', [
							"account_number": GlobalVariable.sender_account_number[i],
							"wallet_number": GlobalVariable.sender_wallet_number[i],
							"card_no": GlobalVariable.activeCardNo[i]			
						]))
					
					def cardClearNo = WS.getElementPropertyValue(GetClearCardNoResponse, "payload.card_no")
					
					def cp_Transaction_PaymentResponse = WS.sendRequestAndVerify(findTestObject('Postman/CardPackage/cp_Transaction_Payment', [
						('ext_transaction_id') : transaction_id,
						('sender_account_number') : GlobalVariable.sender_account_number[i],
						('sender_wallet_number') : GlobalVariable.sender_wallet_number[i]
						, ('currency_code') : "$currency_code",
						 ('amount') : "$amount",
						 ('description') : "$description", ('receiver_wallet_id') : "$receiver_wallet_id"
						, ('media_identifier') : cardClearNo,
						 ('source_type') : "$source_type", ('channel_type') : "$channel_type"
						, ('hash_key') : "$hash_key", ('terminal_no') : "$terminal_no"]))
			
					
					if(WS.getElementPropertyValue(cp_Transaction_PaymentResponse, "status") == 0) {
						// ödeme başarılı
						def correlation_id = WS.getElementPropertyValue(cp_Transaction_PaymentResponse, "payload.transaction_id")
						println("correlation_id : "+correlation_id)
						println("payment_amount : " + "$amount" )
						
						if(GlobalVariable.tx_correlation_id == null) {
							GlobalVariable.tx_correlation_id = []
						}
						GlobalVariable.tx_correlation_id.add(correlation_id)
						
						
						if(GlobalVariable.payment_amount == null) {
							GlobalVariable.payment_amount = []
						}
						GlobalVariable.payment_amount.add("$amount")
					}
					
					CustomKeywords.'writeFile.WriteFile.writeExel'('Payment', " "+"$process_description","$expected_status","$expected_code",
						"$expected_message", fileName="cp_Transaction_Payment","$expected_ResponseStatusCode")
					 
					
					WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"] )
					
					WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"] )
					
					WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"] )
					
					WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"] )
 				
					WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'cp_Transaction_Payment'] )
					 	
				};  
				break; 
				
				
				
				case "passiveCards":
				println(" passiveCards switch içerisinde")
				for(int i=0; i< GlobalVariable.passiveCardNo.size(); i++)
					{
						WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)
						transaction_id = GlobalVariable.uniqe_id
						 
						def GetClearCardNoResponse = WS.sendRequestAndVerify(findTestObject('Postman/CardPackage/GetClearCardNo', [
								"account_number": GlobalVariable.sender_account_number[i],
								"wallet_number": GlobalVariable.sender_wallet_number[i],
								"card_no": GlobalVariable.passiveCardNo[i]
							]))
						
						def cardClearNo = WS.getElementPropertyValue(GetClearCardNoResponse, "payload.card_no")
						
						
						def cp_Transaction_PaymentResponse = WS.sendRequestAndVerify(findTestObject('Postman/CardPackage/cp_Transaction_Payment', [
							('ext_transaction_id') : transaction_id,
							('sender_account_number') : GlobalVariable.sender_account_number[i],
							('sender_wallet_number') : GlobalVariable.sender_wallet_number[i]
							, ('currency_code') : "$currency_code",
							 ('amount') : "$amount",
							 ('description') : "$description", ('receiver_wallet_id') : "$receiver_wallet_id"
							, ('media_identifier') : cardClearNo,
							 ('source_type') : "$source_type", ('channel_type') : "$channel_type"
							, ('hash_key') : "$hash_key", ('terminal_no') : "$terminal_no"]))
				
						
						if(WS.getElementPropertyValue(cp_Transaction_PaymentResponse, "status") == 0) {
							// ödeme başarılı
							def correlation_id = WS.getElementPropertyValue(cp_Transaction_PaymentResponse, "payload.transaction_id")
							println("correlation_id : "+correlation_id)
							println("payment_amount : " + "$amount" )
							
							if(GlobalVariable.tx_correlation_id == null) {
								GlobalVariable.tx_correlation_id = []
							}
							GlobalVariable.tx_correlation_id.add(correlation_id)
							
							
							if(GlobalVariable.payment_amount == null) {
								GlobalVariable.payment_amount = []
							}
							GlobalVariable.payment_amount.add("$amount")
						}
						
						CustomKeywords.'writeFile.WriteFile.writeExel'('Payment', " "+"$process_description","$expected_status","$expected_code",
							"$expected_message", fileName="cp_Transaction_Payment","$expected_ResponseStatusCode")
						 
						
						WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"] )
						
						WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"] )
						
						WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"] )
						
						WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"] )
					 
						WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'cp_Transaction_Payment'] )
							 
					};
					break;
				
			 
					
					case "lostCards":
					println(" lostCards switch içerisinde")
					for(int i=0; i< GlobalVariable.lostCardNo.size(); i++)
						{
							WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)
							transaction_id = GlobalVariable.uniqe_id
							
							
							def GetClearCardNoResponse = WS.sendRequestAndVerify(findTestObject('Postman/CardPackage/GetClearCardNo', [
									"account_number": GlobalVariable.sender_account_number[i],
									"wallet_number": GlobalVariable.sender_wallet_number[i],
									"card_no": GlobalVariable.lostCardNo[i]
								]))
							
							def cardClearNo = WS.getElementPropertyValue(GetClearCardNoResponse, "payload.card_no")
							
							
							
							def cp_Transaction_PaymentResponse = WS.sendRequestAndVerify(findTestObject('Postman/CardPackage/cp_Transaction_Payment', [
								('ext_transaction_id') : transaction_id,
								('sender_account_number') : GlobalVariable.sender_account_number[i],
								('sender_wallet_number') : GlobalVariable.sender_wallet_number[i]
								, ('currency_code') : "$currency_code",
								 ('amount') : "$amount",
								 ('description') : "$description", ('receiver_wallet_id') : "$receiver_wallet_id"
								, ('media_identifier') : cardClearNo,
								 ('source_type') : "$source_type", ('channel_type') : "$channel_type"
								, ('hash_key') : "$hash_key", ('terminal_no') : "$terminal_no"]))
					
							
							if(WS.getElementPropertyValue(cp_Transaction_PaymentResponse, "status") == 0) {
								// ödeme başarılı
								def correlation_id = WS.getElementPropertyValue(cp_Transaction_PaymentResponse, "payload.transaction_id")
								println("correlation_id : "+correlation_id)
								println("payment_amount : " + "$amount" )
								
								if(GlobalVariable.tx_correlation_id == null) {
									GlobalVariable.tx_correlation_id = []
								}
								GlobalVariable.tx_correlation_id.add(correlation_id)
								
								
								if(GlobalVariable.payment_amount == null) {
									GlobalVariable.payment_amount = []
								}
								GlobalVariable.payment_amount.add("$amount")
							}
							
							CustomKeywords.'writeFile.WriteFile.writeExel'('Payment', " "+"$process_description","$expected_status","$expected_code",
								"$expected_message", fileName="cp_Transaction_Payment","$expected_ResponseStatusCode")
							 
							
							WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"] )
							
							WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"] )
							
							WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"] )
							
							WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"] )
						 
							WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'cp_Transaction_Payment'] )
								 
						};
						break;
		 				
						
						case "stolenCards":
						println(" stolenCards switch içerisinde")
						for(int i=0; i< GlobalVariable.stolenCardNo.size(); i++)
							{
								WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)
								transaction_id = GlobalVariable.uniqe_id
								
								def GetClearCardNoResponse = WS.sendRequestAndVerify(findTestObject('Postman/CardPackage/GetClearCardNo', [
									"account_number": GlobalVariable.sender_account_number[i],
									"wallet_number": GlobalVariable.sender_wallet_number[i],
									"card_no": GlobalVariable.stolenCardNo[i]
								]))
							
							def cardClearNo = WS.getElementPropertyValue(GetClearCardNoResponse, "payload.card_no")
							
								
								def cp_Transaction_PaymentResponse = WS.sendRequestAndVerify(findTestObject('Postman/CardPackage/cp_Transaction_Payment', [
									('ext_transaction_id') : transaction_id,
									('sender_account_number') : GlobalVariable.sender_account_number[i],
									('sender_wallet_number') : GlobalVariable.sender_wallet_number[i]
									, ('currency_code') : "$currency_code",
									 ('amount') : "$amount",
									 ('description') : "$description", ('receiver_wallet_id') : "$receiver_wallet_id"
									, ('media_identifier') :cardClearNo, 
									('source_type') : "$source_type", ('channel_type') : "$channel_type"
									, ('hash_key') : "$hash_key", ('terminal_no') : "$terminal_no"]))
						
								
								if(WS.getElementPropertyValue(cp_Transaction_PaymentResponse, "status") == 0) {
									// ödeme başarılı
									def correlation_id = WS.getElementPropertyValue(cp_Transaction_PaymentResponse, "payload.transaction_id")
									println("correlation_id : "+correlation_id)
									println("payment_amount : " + "$amount" )
									
									if(GlobalVariable.tx_correlation_id == null) {
										GlobalVariable.tx_correlation_id = []
									}
									GlobalVariable.tx_correlation_id.add(correlation_id)
									
									
									if(GlobalVariable.payment_amount == null) {
										GlobalVariable.payment_amount = []
									}
									GlobalVariable.payment_amount.add("$amount")
								}
								
								CustomKeywords.'writeFile.WriteFile.writeExel'('Payment', " "+"$process_description","$expected_status","$expected_code",
									"$expected_message", fileName="cp_Transaction_Payment","$expected_ResponseStatusCode")
								 
								
								WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"] )
								
								WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"] )
								
								WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"] )
								
								WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"] )
							 
								WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'cp_Transaction_Payment'] )
									 
							};
							break;
							
							
							
							
							
							case "closedPermanentlyCards":
							for(int i=0; i< GlobalVariable.closedPermanentlyCardNo.size(); i++)
								{
									WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)
									transaction_id = GlobalVariable.uniqe_id
									
									def GetClearCardNoResponse = WS.sendRequestAndVerify(findTestObject('Postman/CardPackage/GetClearCardNo', [
										"account_number": GlobalVariable.sender_account_number[i],
										"wallet_number": GlobalVariable.sender_wallet_number[i],
										"card_no": GlobalVariable.closedPermanentlyCardNo[i]
									]))
								
								def cardClearNo = WS.getElementPropertyValue(GetClearCardNoResponse, "payload.card_no")
								
									
									def cp_Transaction_PaymentResponse = WS.sendRequestAndVerify(findTestObject('Postman/CardPackage/cp_Transaction_Payment', [
										('ext_transaction_id') : transaction_id,
										('sender_account_number') : GlobalVariable.sender_account_number[i],
										('sender_wallet_number') : GlobalVariable.sender_wallet_number[i]
										, ('currency_code') : "$currency_code",
										 ('amount') : "$amount",
										 ('description') : "$description", ('receiver_wallet_id') : "$receiver_wallet_id"
										, ('media_identifier') : cardClearNo,
										('source_type') : "$source_type", ('channel_type') : "$channel_type"
										, ('hash_key') : "$hash_key", ('terminal_no') : "$terminal_no"]))
							
									
									if(WS.getElementPropertyValue(cp_Transaction_PaymentResponse, "status") == 0) {
										// ödeme başarılı
										def correlation_id = WS.getElementPropertyValue(cp_Transaction_PaymentResponse, "payload.transaction_id")
										println("correlation_id : "+correlation_id)
										println("payment_amount : " + "$amount" )
										
										if(GlobalVariable.tx_correlation_id == null) {
											GlobalVariable.tx_correlation_id = []
										}
										GlobalVariable.tx_correlation_id.add(correlation_id)
										
										
										if(GlobalVariable.payment_amount == null) {
											GlobalVariable.payment_amount = []
										}
										GlobalVariable.payment_amount.add("$amount")
									}
									
									CustomKeywords.'writeFile.WriteFile.writeExel'('Payment', " "+"$process_description","$expected_status","$expected_code",
										"$expected_message", fileName="cp_Transaction_Payment","$expected_ResponseStatusCode")
									 
									
									WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"] )
									
									WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"] )
									
									WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"] )
									
									WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"] )
								 
									WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'cp_Transaction_Payment'] )
										 
								};
								break;
		}
		
		
		
	}
}
	
 



