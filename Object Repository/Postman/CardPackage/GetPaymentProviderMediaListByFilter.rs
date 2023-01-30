<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GetPaymentProviderMediaListByFilter</name>
   <tag></tag>
   <elementGuidId>1bf427e0-ba41-498b-8cef-3a9f70f3ffa8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;account_number\&quot;: \&quot;${account_number}\&quot;,\n  \&quot;wallet_number\&quot;: \&quot;${wallet_number}\&quot;,\n  \&quot;card_no_first_6_digits\&quot;: \&quot;${card_no_first_6_digits}\&quot;,\n  \&quot;card_no_last_4_digits\&quot;: \&quot;${card_no_last_4_digits}\&quot;,\n  \&quot;media_type\&quot;: \&quot;${media_type}\&quot;,\n  \&quot;status\&quot;: \&quot;${status}\&quot;,\n  \&quot;provider_id\&quot;: \&quot;${provider_id}\&quot;,\n  \&quot;page_size\&quot;: \&quot;${page_size}\&quot;,\n  \&quot;page_index\&quot;: \&quot;${page_index}\&quot;,\n  \&quot;order_column\&quot;: \&quot;${order_column}\&quot;,\n  \&quot;order_by\&quot;: \&quot;${order_by}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>c2bf3a45-a229-45ed-80a6-2bd17827313b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>76087825-4522-4bc5-86cc-40d17848af82</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/cp/Issuing/GetPaymentProviderMediaListByFilter</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.sts_host</defaultValue>
      <description></description>
      <id>ab8742c1-ae82-44cc-867c-399ddc8bcee4</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>828bd267-3fdc-4c00-8f1d-902b0680534a</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>7e3f473b-04d5-4890-9004-9a7291629349</id>
      <masked>false</masked>
      <name>account_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5b06a4bf-0488-497c-8a46-1ddacfba2941</id>
      <masked>false</masked>
      <name>wallet_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>c9ad2beb-93f7-4b5e-84d3-63df4ef993ad</id>
      <masked>false</masked>
      <name>card_no_first_6_digits</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e15a9d7d-3a62-4758-aefb-62ca8aaae0e9</id>
      <masked>false</masked>
      <name>card_no_last_4_digits</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>473bc2fe-1c49-47ed-afd4-0235c61856ea</id>
      <masked>false</masked>
      <name>media_type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>080058ee-85aa-4c85-a1eb-2bba0c8fff1f</id>
      <masked>false</masked>
      <name>status</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3f156885-6787-43aa-9369-5ceadab9f2ff</id>
      <masked>false</masked>
      <name>provider_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>99745aee-8710-4c89-886b-278d37a28e2e</id>
      <masked>false</masked>
      <name>page_size</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d05310dc-fa39-4e0c-9497-756831b23124</id>
      <masked>false</masked>
      <name>page_index</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b72aaeea-cf8c-42e5-bffb-f4684d1582f6</id>
      <masked>false</masked>
      <name>order_by</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9b67c7ca-d78c-4ed6-bdf5-4158e2b7b018</id>
      <masked>false</masked>
      <name>order_column</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable


RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
 
println(&quot;GetPaymentProviderMediaListByFilter request body: \n&quot;+ request.getBodyContent().getText())

 
GlobalVariable.responseObject = response
GlobalVariable.requestObject = request


if(WS.getResponseStatusCode(response) == 400) {
	WS.verifyResponseStatusCode(response, 400)
}
if(WS.getResponseStatusCode(response) == 200) {
	WS.verifyResponseStatusCode(response, 200)
}

import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText()); // Convert text to object
println(&quot;GetPaymentProviderMediaListByFilter response body\n&quot;+json.toString(4)); // Print it with specified indentation
 
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
