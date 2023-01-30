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

if( (!isNullOrEmpty("${wallet_number}") || "${wallet_number}" !="123" ) && (!isNullOrEmpty("${account_number}") || "${account_number}" !="123" )) 
{
	def walletInfoResponse = WS.sendRequestAndVerify(findTestObject('Postman/Wallet/WalletInfo',
		[
			"wallet_number":"$wallet_number",
			"account_number": "$account_number"
		]))
	
	
	def fileName =  "işlem öncesi WalletInfo - "+"$serviceName"
	def beforeProcess = "islem oncesi\n"
	if("$beforeTransaction" != '1') {
		beforeProcess = "islem sonrasi\n"
		fileName =  "işlem sonrası WalletInfo - "+"$serviceName"
	}
	
	
//	("TopupCash", $process_description, $expected_status, $expected_code, $expected_message, fileName = $file_name, $expected_ResponseStatusCode)
	
	//  serviceName, processDescription,  expected_status, expected_code,  expected_message
	CustomKeywords.'writeFile.WriteFile.writeExel'("$serviceName", beforeProcess + "$description", "", "",
		"", fileName, "")
	
}
