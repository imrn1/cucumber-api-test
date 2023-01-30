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


def order_id = "$ext_transaction_id"

if (isNullOrEmpty(order_id)) {
	WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)

	order_id = GlobalVariable.uniqe_id
} else {
	order_id = ''
}

def CreateQRPersonalToPersonalTransferResponse = WS.sendRequestAndVerify(findTestObject('Postman/Transaction/CreateQRPersonalToPersonalTransfer', [
	   "ext_transaction_id": order_id,
	   "receiver_wallet_number": "${receiver_wallet_number}",
	   "account_number": "${CreateQRPersonalToPersonalTransfer_account_number}",
	   "amount": "${amount}",
	   "currency_code": "${currency_code}",
	   "description": "${description}"
   	]))

CustomKeywords.'writeFile.WriteFile.writeExel'('CreateQRPersonalToPersonalTransfer', "$process_description", "$expected_status", 
	"$expected_code", "$expected_message", fileName = 'QRPersonalToPersonalTransfer işlemleri',
	"$expected_ResponseStatusCode")

WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$expected_status"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$expected_code"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$expected_message"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$expected_ResponseStatusCode"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'CreateQRPersonalToPersonalTransfer'])

if (WS.getElementPropertyValue(CreateQRPersonalToPersonalTransferResponse, 'status') == 0) {
	// Qr oluşturuldu
	def QRCode = WS.getElementPropertyValue(CreateQRPersonalToPersonalTransferResponse, 'payload.qr_code') 
	
	// read QR code
	WS.sendRequestAndVerify(findTestObject('Postman/Transaction/ReadQRPersonalToPersonalTransfer', [('qr_code') : QRCode]))
	

	CustomKeywords.'writeFile.WriteFile.writeExel'('ReadQRPersonalToPersonalTransfer', "$process_description", "$readQrP2P_expected_status",
		"$readQrP2P_expected_code",	"$readQrP2P_expected_message", fileName = 'QRPersonalToPersonalTransfer işlemleri',
		"$readQrP2P_expected_ResponseStatusCode")

	WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$readQrP2P_expected_status"] )
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$readQrP2P_expected_code"] )
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$readQrP2P_expected_message"] )
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$readQrP2P_expected_ResponseStatusCode"] )
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'ReadQRPersonalToPersonalTransfer'])

	
	//CheckQRPaymentStatus
	WS.sendRequestAndVerify(findTestObject('Postman/Transaction/CheckQRPersonalToPersonalTransfer', [('qr_code') : QRCode]))


	CustomKeywords.'writeFile.WriteFile.writeExel'('CheckQRPersonalToPersonalTransfer', "$process_description", "$checkQrP2P_expected_status", "$checkQrP2P_expected_code",
		"$checkQrP2P_expected_message", fileName = 'QRPersonalToPersonalTransfer işlemleri',"$checkQrP2P_expected_ResponseStatusCode")

	WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$checkQrP2P_expected_status"] )
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$checkQrP2P_expected_code"] )
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$checkQrP2P_expected_message"] )
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$checkQrP2P_expected_ResponseStatusCode"] )

	WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'CheckQRPersonalToPersonalTransfer'])

	// ödeme işlemi yapılır
	WS.sendRequestAndVerify(findTestObject('Postman/Transaction/ApproveQRPersonalToPersonalTransfer', [
		"qr_code": QRCode,
		 "account_number": "${ApproveQRPersonalToPersonalTransfer_account_number}",
		"sender_wallet_number": "${sender_wallet_number}"
	 ]))
	
	
	CustomKeywords.'writeFile.WriteFile.writeExel'('ApproveQRPersonalToPersonalTransfer', "$process_description", "$approveQrP2P_expected_status", "$approveQrP2P_expected_code",
		"$approveQrP2P_expected_message", fileName = 'QRPersonalToPersonalTransfer işlemleri',"$approveQrP2P_expected_ResponseStatusCode")

	WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$approveQrP2P_expected_status"] )
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$approveQrP2P_expected_code"] )
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$approveQrP2P_expected_message"] )
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$approveQrP2P_expected_ResponseStatusCode"] )
 
	WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'ApproveQRPersonalToPersonalTransfer'])
	
	
	//ödeme sonrası CheckQRPaymentStatus
	WS.sendRequestAndVerify(findTestObject('Postman/Transaction/CheckQRPersonalToPersonalTransfer', [('qr_code') : QRCode]))

	
	CustomKeywords.'writeFile.WriteFile.writeExel'('CheckQRPersonalToPersonalTransfer',"ödeme sonrası  "+"$process_description", "$lastCheckQrP2P_expected_status", "$lastCheckQrP2P_expected_code",
		"$lastCheckQrP2P_expected_message", fileName = 'QRPersonalToPersonalTransfer işlemleri',"$lastCheckQrP2P_expected_ResponseStatusCode")
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$lastCheckQrP2P_expected_status"] )
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$lastCheckQrP2P_expected_code"] )
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$lastCheckQrP2P_expected_message"] )
	
	WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$lastCheckQrP2P_expected_ResponseStatusCode"] )
   
	
}
 

static boolean isNullOrEmpty(String str) {
	return (str == null) || str.allWhitespace
}
