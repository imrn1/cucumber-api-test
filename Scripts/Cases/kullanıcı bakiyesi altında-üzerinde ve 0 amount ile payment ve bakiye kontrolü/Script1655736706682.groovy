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
import java.time.format.DateTimeFormatter as DateTimeFormatter
import java.time.LocalDateTime as LocalDateTime

static boolean isNullOrEmpty(String str) { return (str == null || str.allWhitespace) }

WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)


def BeforeWalletInfoResponse = WS.sendRequestAndVerify(findTestObject('Postman/Wallet/WalletInfo', [('account_number') : "$sender_account_number"
            , ('wallet_number') : "$sender_wallet_number"]))

// String serviceName="" , String processDescription="", String expected_status="" 
// String expected_code="", String expected_message="", String fileName ="" 
CustomKeywords.'writeFile.WriteFile.writeExel'('WalletInfo', 'payment den önce WalletInfo servis çağrısı',"","","",
	fileName ="payment bakiye kontrolü","200")

def amount = "$amount"
if(isNullOrEmpty("$amount")) {
	def total = WS.getElementPropertyValue(BeforeWalletInfoResponse, "payload.wallet_info.total_balance") + 1
	amount = total.toString()
}

def PaymentResponse = WS.sendRequestAndVerify(findTestObject('Postman/Transaction/Payment', [
	('ext_transaction_id') : GlobalVariable.uniqe_id
            , ('sender_account_number') : "$sender_account_number", ('sender_wallet_number') : "$sender_wallet_number", ('currency_code') : "$currency_code"
            , ('amount') : amount, ('description') : "$description", ('receiver_wallet_number') : "$receiver_wallet_number"
            , ('earn') : "$earn", ('burn') : "$burn", ('hash_key') : "$hash_key", ('channel_type') : "$channel_type", ('source_type') : "$source_type"]))

//  serviceName, processDescription,  expected_status, expected_code,  expected_message
CustomKeywords.'writeFile.WriteFile.writeExel'('Payment', "$process_description", "$Payment_expected_status",
	"","",fileName ="payment bakiye kontrolü")

def AfterWalletInfoResponse = WS.sendRequestAndVerify(findTestObject('Postman/Wallet/WalletInfo', [('account_number') : "$sender_account_number"
            , ('wallet_number') : "$sender_wallet_number"]))

//  serviceName, processDescription,  expected_status, expected_code,  expected_message
CustomKeywords.'writeFile.WriteFile.writeExel'('WalletInfo', 'payment den sonra WalletInfo servis çağrısı',"","","",fileName ="payment bakiye kontrolü")

if (WS.getElementPropertyValue(PaymentResponse, 'status') == 0) {
    def beforeTotalBalance = WS.getElementPropertyValue(BeforeWalletInfoResponse, 'payload.wallet_info.total_balance')

    def paymentResponseTotalBalance = WS.getElementPropertyValue(PaymentResponse, 'payload.sender_wallet_info.total_balance')

    def afterTotalBalance = WS.getElementPropertyValue(AfterWalletInfoResponse, 'payload.wallet_info.total_balance')

    println('beforeTotalBalance : ' + beforeTotalBalance)

    println('paymentResponseTotalBalance : ' + paymentResponseTotalBalance)

    println('afterTotalBalance : ' + afterTotalBalance)

    //String.format("%.2f", 11.545)  ==> bu yapı kullanılmalı 
    def x = String.format('%.2f', beforeTotalBalance - Double.parseDouble("$amount".toString()))

    def y = String.format('%.2f', paymentResponseTotalBalance)

    println((('x: ' + x) + ' y: ') + y)

    WS.verifyEqual(x, y)
}

// expected_status != null ise durum doğrulaması yapılır
WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$BeforeWalletInfo_expected_status"])

WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$AfterWalletInfo_expected_status"])

WebUI.callTestCase(findTestCase('_Verify Cases/verify status'), [('status') : "$Payment_expected_status"])

