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

def order_id = "$ext_order_id"

if (isNullOrEmpty(order_id)) {
    WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)

    order_id = GlobalVariable.uniqe_id
} else {
    order_id = ''
}

/*********************************************/

def CreateQRPaymentWalletNumber = ""
def CreateQRPaymentAccountNumber = "$CreateQRPayment_account_number"
if(!isNullOrEmpty("$CreateQRPayment_account_number") && "$CreateQRPayment_account_number" != "123") {
	
	WebUI.callTestCase(findTestCase('_Custom Cases/set sender_account--sender_wallet--receiver_wallet'),
		[
			('name_for_sender_account_number') : CreateQRPaymentAccountNumber,
			('name_for_sender_wallet_number') : '',
			('name_for_receiver_wallet_number') : ''
		])
	CreateQRPaymentAccountNumber = GlobalVariable.default_sender_account_number
	CreateQRPaymentWalletNumber = GlobalVariable.wallet_number
}

def receiverAccountNumber = ""
def receiverWalletNumber = "$receiver_wallet_number"
if(!isNullOrEmpty("$receiver_wallet_number") && "$receiver_wallet_number" != "123") {
	WebUI.callTestCase(findTestCase('_Custom Cases/set sender_account--sender_wallet--receiver_wallet'),
		[
			('name_for_sender_account_number') : '',
			('name_for_sender_wallet_number') : '',
			('name_for_receiver_wallet_number') : receiverWalletNumber
		])
	receiverWalletNumber = GlobalVariable.default_receiver_wallet_number
	receiverAccountNumber = GlobalVariable.account_number
}
	
/*********************************************/


def CreateQRPaymentResponse = WS.sendRequestAndVerify(findTestObject('Postman/Transaction/CreateQRPayment', [
		('ext_order_id') : order_id,
	    ('account_number') : CreateQRPaymentAccountNumber, 
		('receiver_wallet_number') : receiverWalletNumber,
	    ('currency_code') : "$currency_code", 
		('amount') : "$amount", 
		('description') : "$description"
	]))

CustomKeywords.'writeFile.WriteFile.writeExel'('CreateQRPayment', "$process_description", "$CreateQRPayment_expected_status", 
	"$CreateQRPayment_expected_code", "$CreateQRPayment_expected_message", 
	fileName = 'QRPayment işlemleri',"$CreateQRPayment_expected_ResponseStatusCode")

WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$CreateQRPayment_expected_status"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify code'), [('expected_code') : "$CreateQRPayment_expected_code"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify message'), [('expected_message') : "$CreateQRPayment_expected_message"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify ResponseStatusCode'), [('expected_ResponseStatusCode') : "$CreateQRPayment_expected_ResponseStatusCode"] )

WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'CreateQRPayment'])

if (WS.getElementPropertyValue(CreateQRPaymentResponse, 'status') == 0) {
    // Qr oluşturuldu
    def QRCode = WS.getElementPropertyValue(CreateQRPaymentResponse, 'payload.qr_code') // ödeme öncesi CheckQRPaymentStatus sorgusu exele yazdırılır
	
	// read QR code
	WS.sendRequestAndVerify(findTestObject('Postman/Transaction/ReadQRPayment', [('qr_code') : QRCode]))
	
	CustomKeywords.'writeFile.WriteFile.writeExel'('ReadQRPayment', "$process_description", "$ReadQRPayment_expected_status", "$ReadQRPayment_expected_code", 
    "$ReadQRPayment_expected_message", fileName = 'QRPayment işlemleri',"$ReadQRPayment_expected_ResponseStatusCode")

	WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'ReadQRPayment'])

	
    //CheckQRPaymentStatus
    WS.sendRequestAndVerify(findTestObject('Postman/Transaction/CheckQRPaymentStatus', [('qr_code') : QRCode]))

    CustomKeywords.'writeFile.WriteFile.writeExel'('CheckQRPaymentStatus', "$process_description","$CheckQRPaymentStatus1_expected_status",
	"$CheckQRPaymentStatus1_expected_code","$CheckQRPaymentStatus1_expected_message", fileName = 'QRPayment işlemleri',
	"$CheckQRPaymentStatus1_expected_ResponseStatusCode")

    WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'CheckQRPaymentStatus'])

	
/*************************************************/	
	
	def ApproveQRPaymentAccountNumber = "$ApproveQRPayment_account_number"
	if(!isNullOrEmpty("$ApproveQRPayment_account_number") && "$ApproveQRPayment_account_number" != "123") {
		WebUI.callTestCase(findTestCase('_Custom Cases/set sender_account--sender_wallet--receiver_wallet'),
			[
				('name_for_sender_account_number') : ApproveQRPaymentAccountNumber,
				('name_for_sender_wallet_number') : '',
				('name_for_receiver_wallet_number') : ''
			])
		ApproveQRPaymentAccountNumber = GlobalVariable.default_sender_account_number
	}
	
	def senderWalletNumber = "$sender_wallet_number"
	if(!isNullOrEmpty("$sender_wallet_number") && "$sender_wallet_number" != "123") {
		WebUI.callTestCase(findTestCase('_Custom Cases/set sender_account--sender_wallet--receiver_wallet'),
			[
				('name_for_sender_account_number') : '',
				('name_for_sender_wallet_number') : senderWalletNumber,
				('name_for_receiver_wallet_number') : ''
			])
		senderWalletNumber = GlobalVariable.default_sender_wallet_number
		
	}
	
/*************************************************/
	
	WebUI.callTestCase(findTestCase('_Custom Cases/Wallet Info'),
		[
			('account_number') : ApproveQRPaymentAccountNumber,
			('wallet_number') : senderWalletNumber,
			('description') : "$process_description"+ "\n sender wallet info",
			('beforeTransaction') : 1,
			('serviceName') : 'ApproveQRPayment'
		])
	
	WebUI.callTestCase(findTestCase('_Custom Cases/Wallet Info'),
		[
			('account_number') : receiverAccountNumber,
			('wallet_number') : receiverWalletNumber,
			('description') : "$process_description"+ "\n receiver wallet info",
			('beforeTransaction') : 1,
			('serviceName') : 'ApproveQRPayment'
		])
	
	
    // ödeme işlemi yapılır
    WS.sendRequestAndVerify(findTestObject('Postman/Transaction/ApproveQRPayment', [ 
		"qr_code": QRCode,
 		"account_number": ApproveQRPaymentAccountNumber,
		"sender_wallet_number": senderWalletNumber
	 	]))
	
	
	CustomKeywords.'writeFile.WriteFile.writeExel'('ApproveQRPayment', "$process_description", "$ApproveQRPayment_expected_status",
		"$ApproveQRPayment_expected_code", "$ApproveQRPayment_expected_message", fileName = 'QRPayment işlemleri',
		"$ApproveQRPayment_expected_ResponseStatusCode")

	WebUI.callTestCase(findTestCase('_Verify Cases/verify payloadNotBoolean'), [('servisName') : 'ApproveQRPayment'])
	
	
	//ödeme sonrası CheckQRPaymentStatus
	WS.sendRequestAndVerify(findTestObject('Postman/Transaction/CheckQRPaymentStatus', [('qr_code') : QRCode]))

	CustomKeywords.'writeFile.WriteFile.writeExel'('CheckQRPaymentStatus',"ödeme sonrası  "+"$process_description",
		"$CheckQRPaymentStatus2_expected_status", "$CheckQRPaymentStatus2_expected_code",
		"$CheckQRPaymentStatus2_expected_message", fileName = 'QRPayment işlemleri',"$CheckQRPaymentStatus2_expected_ResponseStatusCode")

	
	
	WebUI.callTestCase(findTestCase('_Custom Cases/Wallet Info'),
		[
			('account_number') : ApproveQRPaymentAccountNumber,
			('wallet_number') : senderWalletNumber,
			('description') : "$process_description"+ "\n sender wallet info",
			('beforeTransaction') : 2,
			('serviceName') : 'ApproveQRPayment'
		])
	
	WebUI.callTestCase(findTestCase('_Custom Cases/Wallet Info'),
		[
			('account_number') : receiverAccountNumber,
			('wallet_number') : receiverWalletNumber,
			('description') : "$process_description"+ "\n receiver wallet info",
			('beforeTransaction') : 2,
			('serviceName') : 'ApproveQRPayment'
		])
	
	 
		
}
 

static boolean isNullOrEmpty(String str) {
    return (str == null) || str.allWhitespace
}

