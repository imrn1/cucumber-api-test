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
import customKeyword.CustomMethods as Custom
import com.kms.katalon.core.testobject.ResponseObject
import customKeyword.Verify as Verify



String body = "{'account_number': $account_number }"

//println('body : ' + body)

ResponseObject response = Custom.sendRequest('/v1/Account/GetPersonalAccount', body)


//  expected_value,  response = GlobalVariable.responseObject
Verify.VerifyResponseStatusCode("$expected_ResponseStatusCode",response)

Verify.VerifyElementPropetyValue("$expected_message", "message", response)
Verify.VerifyElementPropetyValue("$expected_status", "status", response)
Verify.VerifyElementPropetyValue("$expected_code", "code", response)


