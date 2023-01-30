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

// payload = dönmeyen responselar var
try {
    if (WS.getElementPropertyValue(GlobalVariable.responseObject, 'payload') != null) {
        // payload = boolean
        if (WS.getElementPropertyValue(GlobalVariable.responseObject, 'payload').getClass().toString() == true.getClass().toString()) {
           
			
			CustomKeywords.'writeFile.WriteFile.boolReturnedServices'("$servisName", 'BoolReturnedServices')
			
			 /*
			DateTimeFormatter dtf = DateTimeFormatter.ofPattern('yyyy-MM-dd-HH')
			LocalDateTime now = LocalDateTime.now()
						
			File file = new File((GlobalVariable.katalon_std_location+'/STS API test/Data Files/logs/bool dönen servisler/' + 'log.txt'))			
			file.append("$servisName" + "\n")
			*/ 
        }
    }
}
catch (Exception ex) {
} 

