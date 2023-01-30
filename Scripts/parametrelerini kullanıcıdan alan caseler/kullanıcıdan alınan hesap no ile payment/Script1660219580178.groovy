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


WebUI.callTestCase(findTestCase('_Custom Cases/_Token alma'), [('token') : GlobalVariable.token], FailureHandling.STOP_ON_FAILURE)

def transaction_id = "$ext_transaction_id"

if (isNullOrEmpty(transaction_id)) {
    WebUI.callTestCase(findTestCase('_Custom Cases/_GenerateUniqeID'), [:], FailureHandling.STOP_ON_FAILURE)

    transaction_id = GlobalVariable.uniqe_id
} else {
    transaction_id = ''
}

// payment örneği

WebUI.callTestCase(findTestCase('_Custom Cases/get user input'), [('description') : 'personal account no', ('arrayName') : null, ('valueName') : "user_input"] )
def personal_account_num = GlobalVariable.user_input

WebUI.callTestCase(findTestCase('_Custom Cases/get user input'), [('description') : 'personal wallet no', ('arrayName') : null, ('valueName') : "user_input"] )
def personal_wallet_num = GlobalVariable.user_input

WebUI.callTestCase(findTestCase('_Custom Cases/get user input'), [('description') : 'bussines wallet no', ('arrayName') : null, ('valueName') : "user_input"] )
def bussines_wallet_num = GlobalVariable.user_input


WebUI.callTestCase(findTestCase('Transactions/Payment'), [('ext_transaction_id') : transaction_id, ('sender_wallet_number') : "$sender_wallet_number" == 
        'x' ? personal_wallet_num : "$sender_wallet_number", ('sender_account_number') : "$sender_account_number" == 'x' ? personal_account_num : "$sender_account_number"
        , ('currency_code') : "$currency_code", ('amount') : "$amount", ('description') : "$description", ('receiver_wallet_number') : "$receiver_wallet_number" == 
        'x' ? bussines_wallet_num : "$receiver_wallet_number", ('earn') : "$earn", ('burn') : "$burn", ('hash_key') : "$hash_key"
        , ('channel_type') : "$channel_type", ('source_type') : "$source_type", ('expected_status') : "$expected_status"
        , ('expected_code') : "$expected_code", ('expected_message') : "$expected_message", ('process_description') : "$process_description"
        , ('expected_ResponseStatusCode') : "$expected_ResponseStatusCode", ('fileName') : 'kullanıcıdan alınan veriler ile payment'])




